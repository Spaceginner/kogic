use crate::component::{Updater};

#[derive(Default)]
pub struct AND;

impl Updater for AND {
    fn input_count(&self) -> usize {
        2
    }

    fn output_count(&self) -> usize {
        1
    }

    fn name(&self) -> &str {
        "AND"
    }

    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![input[0] && input[1]]
    }
}

pub struct Clock {
    cycle_length: u32,
    count: u32
}

impl Clock {
    pub fn new(cycle_length: u32) -> Self {
        Self { cycle_length, count: 0 }
    }
}

impl Updater for Clock {
    fn input_count(&self) -> usize {
        0
    }
    
    fn output_count(&self) -> usize {
        1
    }

    fn name(&self) -> &str {
        "Clock"
    }

    fn update(&mut self, _input: &[bool]) -> Vec<bool> {
        self.count += 1;
        
        vec![
            if self.count == self.cycle_length {
                self.count = 0;
                true
            } else {
                false
            }
        ]
    }

    fn always_update(&self) -> bool {
        true
    }
}
