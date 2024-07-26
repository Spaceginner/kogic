pub struct Component {
    state: Vec<bool>,
    next_state: Vec<bool>,
    connected_to: Vec<usize>,
    depends_on: Vec<(usize, usize)>,
    updater: Box<dyn Updater>,
}


impl Component {
    pub fn new<U: Updater + 'static>(updater: U, depends_on: &[(usize, usize)]) -> Self {
        assert_eq!(updater.input_count(), depends_on.len());

        Self {
            state: vec![false; updater.output_count()],
            next_state: Vec::new(),
            connected_to: Vec::new(),
            depends_on: depends_on.to_vec(),
            updater: Box::new(updater)
        }
    }
    
    pub fn state(&self) -> &[bool] {
        &self.state
    }
    
    pub fn updater(&self) -> &dyn Updater {
        self.updater.as_ref()
    }
    
    pub fn depends_on(&self) -> &[(usize, usize)] {
        &self.depends_on
    }
    
    pub fn connected_to(&self) -> &[usize] {
        &self.connected_to
    }
    
    pub fn register_connection(&mut self, index: usize) {
        self.connected_to.push(index);
    }
    
    pub fn update(&mut self, states: &[Vec<bool>]) {
        self.next_state = self.updater.update(&self.depends_on.iter().copied().map(|c| states[c.0][c.1]).collect::<Vec<_>>());
    }  
    
    pub fn finalize(&mut self) -> bool {
        assert_ne!(0, self.next_state.len());
        
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

    fn input_count(&self) -> usize;
    fn output_count(&self) -> usize;

    fn name(&self) -> &str;
    
    fn always_update(&self) -> bool {
        false
    } 
}
