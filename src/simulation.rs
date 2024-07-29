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
    
    fn get_states(components: &esnarl::Snarl<Component>) -> HashMap<esnarl::NodeId, Vec<Vec<bool>>> {
        HashMap::from_iter(components.node_ids().map(|(i, comp)| (i, comp.state.clone())))
    }
    
    pub fn tick(&mut self, components: &mut esnarl::Snarl<Component>) {
        let states = Self::get_states(components);
        
        self.queue.iter().for_each(|i| components[*i].update(&states));

        let mut next_queue = Vec::new();
        while let Some((i, comp)) = self.queue.pop().map(|i| (i, &mut components[i])) {
            if comp.finalize() {
                next_queue.append(&mut comp.connected_to.clone());
            };
            
            if comp.updater.force_update() {
                next_queue.push(i);
            };
        };
        
        self.queue = next_queue;
    }
}
