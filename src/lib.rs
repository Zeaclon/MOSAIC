mod mosaic;

pub use mosaic::configuration::{
    Configuration, Monitor, MonitorMode, MonitorPosition, MonitorScale, Rotation,
};

pub fn initialize() {
    mosaic::initialize();
}

pub fn render_hyprland_configuration(configuration: &Configuration) -> String {
    mosaic::providers::hyprland::render_configuration(configuration)
}
