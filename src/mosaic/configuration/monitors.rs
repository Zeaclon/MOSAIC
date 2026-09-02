#[derive(PartialEq)]
pub enum MonitorMode {
    Preferred,
    HighResolution,
    HighRefreshRate,
    MaxWidth,
    Custom(String),
}

impl Default for MonitorMode {
    fn default() -> Self {
        Self::Preferred
    }
}

#[derive(PartialEq)]
pub enum MonitorScale {
    Auto,
    Fixed(f32),
}

impl Default for MonitorScale {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(PartialEq)]
pub enum MonitorPosition {
    Auto,
    AutoRight,
    AutoLeft,
    AutoUp,
    AutoDown,
    AutoCenterRight,
    AutoCenterLeft,
    AutoCenterUp,
    AutoCenterDown,
    Fixed { x: i32, y: i32 },
}

impl Default for MonitorPosition {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(PartialEq)]
pub enum Rotation {
    Degrees0,
    Degrees90,
    Degrees180,
    Degrees270,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Degrees0
    }
}

pub struct Monitor {
    pub output: String,
    pub disabled: bool,
    pub mode: MonitorMode,
    pub scale: MonitorScale,
    pub position: MonitorPosition,
    pub rotation: Rotation,
    pub flip: bool,
    pub mirror: Option<String>,
}

impl Default for Monitor {
    fn default() -> Self {
        Self {
            output: String::new(),
            disabled: false,
            mode: MonitorMode::default(),
            scale: MonitorScale::default(),
            position: MonitorPosition::default(),
            rotation: Rotation::default(),
            flip: false,
            mirror: None,
        }
    }
}
