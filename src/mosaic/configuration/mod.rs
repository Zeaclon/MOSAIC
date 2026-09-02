pub(crate) mod monitors;

pub use monitors::{Monitor, MonitorMode, MonitorPosition, MonitorScale, Rotation};

pub struct Configuration {
    pub monitors: Vec<Monitor>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
        }
    }
}

pub fn initialize() {
    let _configuration = Configuration::new();

    println!("MOSAIC configuration initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_can_be_created() {
        let _configuration = Configuration::new();
    }
}
