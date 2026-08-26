mod components;
mod configuration;
mod core;
mod providers;
mod runtime;

pub fn initialize() {
    println!("MOSAIC initialized");

    core::initialize();
    configuration::initialize();
    components::initialize();
    providers::initialize();
    runtime::initialize();
}
