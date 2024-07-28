use std::collections::HashMap;
use egui_snarl as esnarl;
use crate::component::Component;

#[derive(Default)]
pub struct Simulation {
    queue: Vec<esnarl::NodeId>,
}


impl Simulation {
    pub fn add_to_queue(&mut self, index: esnarl::NodeId) {
        self.queue.push(index);
    }
    
    fn get_states(components: &esnarl::Snarl<Component>) -> HashMap<esnarl::NodeId, Vec<bool>> {
        HashMap::from_iter(components.node_ids().map(|(i, comp)| (i, comp.state.clone())))
    }
    
    pub fn tick(&mut self, components: &mut esnarl::Snarl<Component>) {
        let states = Self::get_states(components);
        let mut queue = self.queue.clone();
        
        while let Some(comp) = self.queue.pop().map(|i| &mut components[i]) {
            comp.update(&states);
        };

        while let Some((i, comp)) = queue.pop().map(|i| (i, &mut components[i])) {
            if comp.finalize() {
                self.queue.append(&mut comp.connected_to.clone());
            };
            
            if comp.updater.always_update() {
                self.queue.push(i);
            };
        };
    }
}
