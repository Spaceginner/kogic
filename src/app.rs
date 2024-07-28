use std::any::{Any, TypeId};
use egui_snarl as esnarl;
use crate::component::{Component, Updater};
use crate::components::{AND, Out, Clock, Button};
use crate::simulation::Simulation;

pub struct App {
    snarl: esnarl::Snarl<Component>,
    simulation: Simulation,
}


impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let mut self_ = Self {
            snarl: Default::default(),
            simulation: Simulation::default(),
        };

        self_.simulation.add_to_queue(self_.snarl.insert_node(egui::Pos2::default(), Component::new(Clock::new(1, 1))));
        self_.simulation.add_to_queue(self_.snarl.insert_node(egui::Pos2::default(), Component::new(Clock::new(4, 1))));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(AND));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(AND));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Out));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Button));
        
        
        self_
    }
}


impl esnarl::ui::SnarlViewer<Component> for Simulation {
    fn title(&mut self, node: &Component) -> String {
        node.updater.name().to_string()
    }

    fn inputs(&mut self, node: &Component) -> usize {
        node.updater.input_count()
    }

    fn outputs(&mut self, node: &Component) -> usize {
        node.updater.output_count()
    }

    fn has_body(&mut self, _node: &Component) -> bool {
        true
    }
    
    fn show_body(&mut self, node: esnarl::NodeId, _inputs: &[esnarl::InPin], _outputs: &[esnarl::OutPin], ui: &mut egui::Ui, _scale: f32, snarl: &mut esnarl::Snarl<Component>) {
        let mut comp = &mut snarl[node];
        
        if comp.updater.is_out() {
            if *comp.state.first().unwrap_or(&false) {
                ui.code("on.");
            } else {
                ui.code("off.");
            };
        } else if comp.updater.is_button() {
            comp.state = vec![ui.button("activate.").is_pointer_button_down_on()];
        };
    }

    fn show_input(&mut self, pin: &esnarl::InPin, _ui: &mut egui::Ui, _scale: f32, snarl: &mut esnarl::Snarl<Component>) -> esnarl::ui::PinInfo {
        esnarl::ui::PinInfo::square().with_fill(if !pin.remotes.is_empty() {
            if snarl[pin.remotes[0].node].state[pin.remotes[0].output] { egui::Color32::GREEN } else { egui::Color32::DARK_GRAY }
        } else {
            egui::Color32::LIGHT_GRAY
        })
    }

    fn show_output(&mut self, pin: &esnarl::OutPin, _ui: &mut egui::Ui, _scale: f32, snarl: &mut esnarl::Snarl<Component>) -> esnarl::ui::PinInfo {
        esnarl::ui::PinInfo::circle().with_fill(if snarl[pin.id.node].state[pin.id.output] { egui::Color32::GREEN } else { egui::Color32::DARK_GRAY })
    }

    fn connect(&mut self, from: &esnarl::OutPin, to: &esnarl::InPin, snarl: &mut esnarl::Snarl<Component>) {
        snarl[from.id.node].connected_to.push(to.id.node);
        snarl[to.id.node].depends_on.push((from.id, to.id.input));
        if snarl[from.id.node].state[from.id.output] {
            self.add_to_queue(to.id.node);
        };

        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }
        
        snarl.connect(from.id, to.id);
    }

    fn disconnect(&mut self, from: &esnarl::OutPin, to: &esnarl::InPin, snarl: &mut esnarl::Snarl<Component>) {
        snarl[from.id.node].register_disconnection(to.id.node);
        snarl[to.id.node].register_undepending(to.id.input);
        if snarl[from.id.node].state[from.id.output] {
            self.add_to_queue(to.id.node);
        };
        snarl.disconnect(from.id, to.id);
    }

    // fn show_dropped_wire_menu(&mut self, pos: egui::Pos2, ui: &mut egui::Ui, scale: f32, src_pins: esnarl::ui::AnyPins, snarl: &mut esnarl::Snarl<ComponentNode>) {
    //
    // }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl.show(&mut self.simulation, &esnarl::ui::SnarlStyle::default(), egui::Id::new("snarl"), ui)
        });

        self.simulation.tick(&mut self.snarl)
    }
}
