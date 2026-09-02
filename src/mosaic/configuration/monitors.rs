pub enum MonitorMode {
    Preferred,
    HighResolution,
    HighRefreshRate,
    MaxWidth,
    Custom(String),
}

pub enum MonitorScale {
    Auto,
    Fixed(f32),
}

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

pub enum Rotation {
    Degrees0,
    Degrees90,
    Degrees180,
    Degrees270,
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