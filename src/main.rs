#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::component::Component;
use crate::components::{AND, Clock};
use crate::simulation::Simulation;

mod simulation;
mod component;
mod components;
mod app;

pub struct App {
    simulation: Simulation
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    eframe::run_native(
        "kogic",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::App::new(cc))))
    )
}
