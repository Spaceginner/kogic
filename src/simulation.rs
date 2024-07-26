use crate::component::Component;

#[derive(Default)]
pub struct Simulation {
    components: Vec<Component>,
    queue: Vec<usize>,
}


impl Simulation {
    pub fn add_component(&mut self, comp: Component) -> usize {
        let index = self.components.len();
        
        if comp.updater().always_update() {
            self.queue.push(index);
        }
        
        for comp_index in comp.depends_on().iter().copied() {
            self.components[comp_index.0].register_connection(index);
        };

        self.components.push(comp);
        
        index
    }
    
    pub fn components(&self) -> &[Component] {
        &self.components
    } 
    
    fn get_states(&self) -> Vec<Vec<bool>> {
        self.components.iter().map(|c| c.state().to_vec()).collect::<Vec<_>>()
    }
    
    pub fn tick(&mut self) {
        let states = self.get_states();
        let mut queue = self.queue.clone();
        
        while let Some(comp) = self.queue.pop().map(|i| &mut self.components[i]) {
            comp.update(&states);
        };

        while let Some((i, comp)) = queue.pop().map(|i| (i, &mut self.components[i])) {
            if comp.finalize() {
                self.queue.append(&mut comp.connected_to().to_vec())
            };
            
            if comp.updater().always_update() {
                self.queue.push(i);
            }
        };
    }
}


impl std::fmt::Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.components.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "))
    }
}
