use std::any::Any;
use crate::component::{Updater};

#[derive(Default)]
pub struct AND;

impl Updater for AND {
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![input[0] && input[1]]
    }
}

pub struct Clock {
    off_len: u32,
    cycle_end: u32,
    cycle_state: u32
}

impl Clock {
    pub fn new(off_len: u32, on_len: u32) -> Self {
        Self { off_len, cycle_end: off_len + on_len, cycle_state: 0 }
    }
}

impl Updater for Clock {
    fn input_count(&self) -> usize {
        0
    }

    fn update(&mut self, _input: &[bool]) -> Vec<bool> {
        self.cycle_state += 1;
        
        vec![
            if self.cycle_state < self.off_len {
                false
            } else if self.cycle_state < self.cycle_end {
                true
            } else {
                self.cycle_state = 0;
                true
            }
        ]
    }

    fn always_update(&self) -> bool {
        true
    }
}


pub struct OR;

impl Updater for OR {
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![input[0] || input[1]]
    }
}


pub struct XOR;

impl Updater for XOR {
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![input[0] || input[1]]
    }
}


pub struct NAND;

impl Updater for NAND {
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![!(input[0] && input[1])]
    }
}


pub struct NOR;

impl Updater for NOR {
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![!(input[0] || input[1])]
    }
}


pub struct XNOR;

impl Updater for XNOR {
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![!(input[0] ^ input[1])]
    }
}


pub struct Button;

impl Updater for Button {
    fn input_count(&self) -> usize {
        0
    }
    
    fn update(&mut self, _input: &[bool]) -> Vec<bool> {
        vec![false]
    }

    fn is_button(&self) -> bool {
        true
    }
}


pub struct Out;

impl Updater for Out {
    fn input_count(&self) -> usize {
        1
    }
    
    fn output_count(&self) -> usize {
        0
    }
    
    fn update(&mut self, input: &[bool]) -> Vec<bool> {
        vec![input[0]]
    }

    fn is_out(&self) -> bool {
        true
    }
}
