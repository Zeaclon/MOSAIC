pub(crate) mod components;
pub(crate) mod configuration;
pub(crate) mod core;
pub(crate) mod providers;
pub(crate) mod runtime;

pub fn initialize() {
    println!("MOSAIC initialized");

    core::initialize();
    configuration::initialize();
    components::initialize();
    providers::initialize();
    runtime::initialize();
}
