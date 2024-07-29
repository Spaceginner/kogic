#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod simulation;
mod component;
mod app;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    eframe::run_native(
        "kogic",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::App::new(cc))))
    )
}
