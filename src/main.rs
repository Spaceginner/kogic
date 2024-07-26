use crate::component::Component;
use crate::components::{AND, Clock};
use crate::simulation::Simulation;

mod simulation;
mod component;
mod components;

fn main() {
    let mut sim = Simulation::default();
    
    let clock_1 = sim.add_component(Component::new(Clock::new(2), &[]));
    let clock_2 = sim.add_component(Component::new(Clock::new(5), &[]));
    let and = sim.add_component(Component::new(AND, &[(clock_1, 0), (clock_2, 0)]));
    
    loop {
        sim.tick();
        println!("{sim}");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
