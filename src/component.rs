use std::any::Any;
use std::collections::HashMap;
use egui_snarl as esnarl;

pub struct Component {
    pub(crate) state: Vec<bool>,
    pub(crate) next_state: Vec<bool>,
    pub(crate) connected_to: Vec<esnarl::NodeId>,
    pub(crate) depends_on: Vec<(esnarl::OutPinId, usize)>,
    pub(crate) updater: Box<dyn Updater + 'static>,
}


impl Component {
    pub fn new<U: Updater + 'static>(updater: U) -> Self {
        Self {
            state: vec![false; updater.output_count()],
            next_state: vec![false; updater.output_count()],
            connected_to: Vec::new(),
            depends_on: Vec::new(),
            updater: Box::new(updater)
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

    fn get_input_states(&self, states: &HashMap<esnarl::NodeId, Vec<bool>>) -> Vec<bool> {
        let mut input_states = vec![false; self.updater.input_count()];

        for pin in self.depends_on.iter().copied() {
            input_states[pin.1] = states[&pin.0.node][pin.0.output];
        };

        input_states
    }

    pub fn update(&mut self, states: &HashMap<esnarl::NodeId, Vec<bool>>) {
        let inputs = self.get_input_states(states);
        self.next_state = self.updater.update(&inputs);
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


pub trait Updater {
    fn update(&mut self, input: &[bool]) -> Vec<bool>;

    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }

    fn name(&self) -> &str
        where Self: Any
    {
        let name = std::any::type_name_of_val(self);
        
        &name[name.rfind("::").unwrap()+2..]
    }
    
    fn always_update(&self) -> bool {
        false
    }
    
    // very janky, but it works
    fn is_button(&self) -> bool {
        false
    }
    
    fn is_out(&self) -> bool {
        false
    }
}
