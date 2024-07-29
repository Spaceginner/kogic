use egui::Widget;
use egui_snarl as esnarl;
use crate::component::{Component, Updater};
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

        self_.simulation.add_to_queue(self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::Clock { off_period: 100, cycle_length: 200, state: 0 })));
        self_.simulation.add_to_queue(self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::Clock { off_period: 500, cycle_length: 600, state: 0 })));
        self_.simulation.add_to_queue(self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::Switch { is_toggled: false })));
        self_.simulation.add_to_queue(self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::Switch { is_toggled: false })));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::NAND));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::NAND));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::NAND));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::NAND));
        self_.snarl.insert_node(egui::Pos2::default(), Component::new(Updater::Indicator { name: "Very Important".to_string(), is_lit: false }));
        
        self_
    }
}


impl esnarl::ui::SnarlViewer<Component> for Simulation {
    fn title(&mut self, node: &Component) -> String {
        node.updater.name()
    }

    fn inputs(&mut self, node: &Component) -> usize {
        node.updater.input_count().len()
    }

    fn outputs(&mut self, node: &Component) -> usize {
        node.updater.output_count().len()
    }

    fn has_body(&mut self, _node: &Component) -> bool {
        true
    }
    
    fn show_body(&mut self, node: esnarl::NodeId, inputs: &[esnarl::InPin], _outputs: &[esnarl::OutPin], ui: &mut egui::Ui, _scale: f32, snarl: &mut esnarl::Snarl<Component>) {
        match &mut snarl[node].updater {
            Updater::Indicator { is_lit, .. } =>  {
                if *is_lit {
                    ui.code("on.");
                } else {
                    ui.code("off.");
                };
            },
            Updater::Button { is_pressed } => {
                *is_pressed = ui.button("activate.").is_pointer_button_down_on();
            },
            Updater::Switch { is_toggled } => {
                *is_toggled ^= ui.button("switch.").clicked();
            },
            Updater::Decompositor { split_into } => {
                ui.label("split into");
                egui::DragValue::new(split_into).range(0..=inputs[0].remotes.len()).ui(ui);
            }
            _ => {},
        }
    }

    fn show_input(&mut self, pin: &esnarl::InPin, ui: &mut egui::Ui, _scale: f32, snarl: &mut esnarl::Snarl<Component>) -> esnarl::ui::PinInfo {
        // fixme for multi-connect ones
        esnarl::ui::PinInfo::square().with_fill(if !pin.remotes.is_empty() {
            if snarl[pin.remotes[0].node].state[pin.remotes[0].output].iter().any(|s| *s) { egui::Color32::GREEN } else { egui::Color32::DARK_GRAY }
        } else {
            egui::Color32::LIGHT_GRAY
        })
    }

    fn show_output(&mut self, pin: &esnarl::OutPin, _ui: &mut egui::Ui, _scale: f32, snarl: &mut esnarl::Snarl<Component>) -> esnarl::ui::PinInfo {
        esnarl::ui::PinInfo::circle().with_fill(if snarl[pin.id.node].state[pin.id.output].iter().any(|s| *s) { egui::Color32::GREEN } else { egui::Color32::DARK_GRAY })
    }

    fn connect(&mut self, from: &esnarl::OutPin, to: &esnarl::InPin, snarl: &mut esnarl::Snarl<Component>) {
        snarl[from.id.node].connected_to.push(to.id.node);
        snarl[to.id.node].depends_on.push((from.id, to.id.input));
        if snarl[from.id.node].state[from.id.output].iter().any(|s| *s) {
            self.add_to_queue(to.id.node);
        };

        match snarl[to.id.node].updater {
            Updater::Compositor { .. } => { },
            _ => for &remote in &to.remotes {
                snarl.disconnect(remote, to.id);
            },
        };
        
        snarl.connect(from.id, to.id);
    }

    fn disconnect(&mut self, from: &esnarl::OutPin, to: &esnarl::InPin, snarl: &mut esnarl::Snarl<Component>) {
        snarl[from.id.node].register_disconnection(to.id.node);
        snarl[to.id.node].register_undepending(to.id.input);
        if snarl[from.id.node].state[from.id.output].iter().any(|s| *s) {
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
