use std::collections::HashMap;
use egui_snarl as esnarl;

pub struct Component {
    pub(crate) state: Vec<Vec<bool>>,
    pub(crate) next_state: Vec<Vec<bool>>,
    pub(crate) depends_on: Vec<(esnarl::OutPinId, usize)>,
    pub(crate) updater: Updater,
    pub(crate) connected_to: Vec<esnarl::NodeId>,
}


impl Component {
    fn construct_init_state(sizes: &[Option<usize>]) -> Vec<Vec<bool>> {
        sizes.iter().map(|c| c.map(|s| vec![false; s]).unwrap_or_default()).collect()
    }

    pub fn new(updater: Updater) -> Self {
        Self {
            state: Self::construct_init_state(&updater.output_count()),
            next_state: Self::construct_init_state(&updater.output_count()),
            connected_to: Vec::new(),
            depends_on: Vec::new(),
            updater
        }
    }

    pub fn register_disconnection(&mut self, index: esnarl::NodeId) {
        if let Some(pos) = self.connected_to.iter().copied().position(|i| i == index) {
            self.connected_to.remove(pos);
        };
    }
    
    pub fn register_undepending(&mut self, index: usize) {
        if let Some(pos) = self.depends_on.iter().copied().position(|i| i.1 == index) {
            self.depends_on.remove(pos);
        };
    }

    fn get_input_states(&self, states: &HashMap<esnarl::NodeId, Vec<Vec<bool>>>) -> Vec<Vec<bool>> {
        let mut input_states = Self::construct_init_state(&self.updater.input_count());

        for pin in self.depends_on.iter().copied() {
            input_states[pin.1].clone_from(&states[&pin.0.node][pin.0.output]);
        };

        input_states
    }

    pub fn update(&mut self, states: &HashMap<esnarl::NodeId, Vec<Vec<bool>>>) {
        let inputs = self.get_input_states(states);
        self.updater.update(&inputs, &mut self.next_state);
    }
    
    pub fn finalize(&mut self) -> bool {
        if self.state == self.next_state {
            return false;
        };
        
        self.state.clone_from(&self.next_state);
        
        true
    }
}


impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.updater.name(), self.state)
    }
}


pub enum Updater {
    Constant { state: bool },
    In { name: String, size: usize, port: usize },
    Out { name: String, size: usize, port: usize },
    Button { is_pressed: bool },
    Switch { is_toggled: bool },
    Indicator { name: String, is_lit: bool },
    Compositor,
    Decompositor { split_into: usize },
    NAND,
    Clock {
        off_period: u32,
        cycle_length: u32,
        state: u32,
    },
    Composite {
        name: String,
        components: esnarl::Snarl<Component>,
        in_ports: Vec<(esnarl::InPinId, usize)>,
        out_ports: Vec<(esnarl::OutPinId, usize)>,
        queue: Vec<esnarl::NodeId>,
    }
}


impl Updater {
    pub fn name(&self) -> String {
        match self {
            Self::Constant { .. } => "Constant".to_string(),
            Self::In { name, .. } => format!("In «{name}»"),
            Self::Out { name, .. } => format!("Out «{name}»"),
            Self::Button { .. } => "Button".to_string(),
            Self::Switch { .. } => "Switch".to_string(),
            Self::Indicator { name, .. } => format!("Indicator «{name}»"),
            Self::Compositor => "Compositor".to_string(),
            Self::Decompositor { .. } => "Decompositor".to_string(),
            Self::NAND => "NAND".to_string(),
            Self::Clock { off_period, cycle_length, .. } => format!("Clock {off_period}/{cycle_length}"),
            Self::Composite { name, .. } => format!("Composite «{name}»"),
        }
    }
    
    pub fn input_count(&self) -> Vec<Option<usize>> {
        match self {
            Self::Constant { .. } => vec![],
            Self::In { .. } => vec![],
            Self::Out { size, .. } => vec![Some(*size)],
            Self::Button { .. } => vec![],
            Self::Switch { .. } => vec![],
            Self::Indicator { .. } => vec![Some(1)],
            Self::Compositor => vec![None],
            Self::Decompositor { .. } => vec![Some(1)],
            Self::NAND => vec![Some(1), Some(1)],
            Self::Clock { .. } => vec![Some(1)],
            Self::Composite { in_ports, .. } => in_ports.iter().map(|(_, size)| Some(*size)).collect(),
        }
    }
    
    pub fn output_count(&self) -> Vec<Option<usize>> {
        match self {
            Self::Constant { .. } => vec![Some(1)],
            Self::In { .. } => vec![Some(1)],
            Self::Out { .. } => vec![],
            Self::Button { .. } => vec![Some(1)],
            Self::Switch { .. } => vec![Some(1)],
            Self::Indicator { .. } => vec![],
            Self::Compositor => vec![],
            Self::Decompositor { split_into } => vec![None; *split_into],
            Self::NAND => vec![Some(1)],
            Self::Clock { .. } => vec![Some(1)],
            Self::Composite { out_ports, .. } => out_ports.iter().map(|(_, size)| Some(*size)).collect(),
        }
    }
    
    pub fn update(&mut self, input: &[Vec<bool>], output: &mut Vec<Vec<bool>>) {
        match self {
            Self::Constant { state } => { output[0][0] = *state; },
            Self::In { .. } => unreachable!("done via Composite's update"),
            Self::Out { .. } => unreachable!("done via Composite's update"),
            Self::Button { is_pressed } => { output[0][0] = *is_pressed; },
            Self::Switch { is_toggled } => { output[0][0] = *is_toggled; },
            Self::Indicator { is_lit, .. } => { *is_lit = input[0][0]; },
            Self::Compositor => { output[0] = input[0].to_vec(); },
            Self::Decompositor { split_into } => {
                if output.len() != *split_into {
                    *output = vec![vec![]; *split_into];
                };

                let per_each = input[0].len() / *split_into;
                let left = input[0].len() % *split_into;

                for of_i in 0..(*split_into-1) {
                    let of = of_i*per_each;
                    output[of_i] = input[0][of..of+per_each].to_vec();
                }

                let last_of = (*split_into-1)*per_each;
                output[*split_into] = input[0][last_of..last_of+left].to_vec();
            },
            Self::NAND => { output[0][0] = !(input[0][0] && input[1][0]); }
            Self::Clock { off_period, cycle_length, state } => {
                if input[0][0] {
                    *state += 1;

                    output[0][0] = if state <= off_period {
                        false
                    } else if state < cycle_length {
                        true
                    } else {
                        *state = 0;
                        true
                    };
                };
            },
            Self::Composite { .. } => todo!("implement composite's update"),
        };
    }

    pub fn force_update(&self) -> bool {
        matches!(self, Self::Button { .. } | Self::Switch { .. } | Self::Clock { .. } | Self::Composite { .. })
    }
}
