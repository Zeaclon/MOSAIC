use crate::mosaic::configuration::monitors::{
    Monitor, MonitorMode, MonitorPosition, MonitorScale, Rotation,
};

pub fn render_monitor(monitor: &Monitor) -> String {
    let mut fields = Vec::new();

    fields.push(format!("output = {}", lua_string(&monitor.output)));
    fields.push(format!("disabled = {}", monitor.disabled));
    fields.push(format!("mode = {}", render_mode(&monitor.mode)));
    fields.push(format!("scale = {}", render_scale(&monitor.scale)));
    fields.push(format!("position = {}", render_position(&monitor.position)));
    fields.push(format!(
        "transform = {}",
        render_transform(&monitor.rotation, monitor.flip)
    ));

    if let Some(mirror) = &monitor.mirror {
        fields.push(format!("mirror = {}", lua_string(mirror)));
    }

    format!("hl.monitor({{ {} }})", fields.join(", "))
}

fn render_mode(mode: &MonitorMode) -> String {
    match mode {
        MonitorMode::Preferred => lua_string("preferred"),
        MonitorMode::HighResolution => lua_string("highres"),
        MonitorMode::HighRefreshRate => lua_string("highrr"),
        MonitorMode::MaxWidth => lua_string("maxwidth"),
        MonitorMode::Custom(mode) => lua_string(mode),
    }
}

fn render_scale(scale: &MonitorScale) -> String {
    match scale {
        MonitorScale::Auto => lua_string("auto"),
        MonitorScale::Fixed(value) => value.to_string(),
    }
}

fn render_position(position: &MonitorPosition) -> String {
    match position {
        MonitorPosition::Auto => lua_string("auto"),
        MonitorPosition::AutoRight => lua_string("auto-right"),
        MonitorPosition::AutoLeft => lua_string("auto-left"),
        MonitorPosition::AutoUp => lua_string("auto-up"),
        MonitorPosition::AutoDown => lua_string("auto-down"),
        MonitorPosition::AutoCenterRight => lua_string("auto-center-right"),
        MonitorPosition::AutoCenterLeft => lua_string("auto-center-left"),
        MonitorPosition::AutoCenterUp => lua_string("auto-center-up"),
        MonitorPosition::AutoCenterDown => lua_string("auto-center-down"),
        MonitorPosition::Fixed { x, y } => lua_string(&format!("{x}x{y}")),
    }
}

fn render_transform(rotation: &Rotation, flip: bool) -> u8 {
    match (rotation, flip) {
        (Rotation::Degrees0, false) => 0,
        (Rotation::Degrees90, false) => 1,
        (Rotation::Degrees180, false) => 2,
        (Rotation::Degrees270, false) => 3,

        (Rotation::Degrees0, true) => 4,
        (Rotation::Degrees90, true) => 5,
        (Rotation::Degrees180, true) => 6,
        (Rotation::Degrees270, true) => 7,
    }
}

fn lua_string(value: &str) -> String {
    format!("{value:?}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mosaic::configuration::monitors::{
        Monitor, MonitorMode, MonitorPosition, MonitorScale, Rotation,
    };

    #[test]
    fn renders_monitor() {
        let monitor = Monitor {
            output: "DP-3".into(),
            disabled: false,
            mode: MonitorMode::Custom("2560x1440@144".into()),
            scale: MonitorScale::Fixed(1.0),
            position: MonitorPosition::Fixed { x: 0, y: 0 },
            rotation: Rotation::Degrees0,
            flip: true,
            mirror: None,
        };

        let result = render_monitor(&monitor);

        assert_eq!(
            result,
            r#"hl.monitor({ output = "DP-3", disabled = false, mode = "2560x1440@144", scale = 1, position = "0x0", transform = 4 })"#
        );
    }

    #[test]
    fn rotation_maps_to_hyprland_transform() {
        assert_eq!(render_transform(&Rotation::Degrees0, false), 0);

        assert_eq!(render_transform(&Rotation::Degrees90, false), 1);

        assert_eq!(render_transform(&Rotation::Degrees180, false), 2);

        assert_eq!(render_transform(&Rotation::Degrees270, false), 3);
    }

    #[test]
    fn horizontal_flip_maps_to_hyprland_transform() {
        assert_eq!(render_transform(&Rotation::Degrees0, true), 4);

        assert_eq!(render_transform(&Rotation::Degrees90, true), 5);

        assert_eq!(render_transform(&Rotation::Degrees180, true), 6);

        assert_eq!(render_transform(&Rotation::Degrees270, true), 7);
    }

    #[test]
    fn renders_monitor_mode() {
        assert_eq!(render_mode(&MonitorMode::Preferred), "\"preferred\"");
        assert_eq!(render_mode(&MonitorMode::HighResolution), "\"highres\"");
        assert_eq!(render_mode(&MonitorMode::HighRefreshRate), "\"highrr\"");
        assert_eq!(render_mode(&MonitorMode::MaxWidth), "\"maxwidth\"");
        assert_eq!(
            render_mode(&MonitorMode::Custom("2560x1440@144".into())),
            "\"2560x1440@144\""
        );
    }

    #[test]
    fn renders_monitor_scale() {
        assert_eq!(render_scale(&MonitorScale::Auto), "\"auto\"");
        assert_eq!(render_scale(&MonitorScale::Fixed(1.5)), "1.5");
    }

    #[test]
    fn renders_monitor_position() {
        assert_eq!(render_position(&MonitorPosition::Auto), "\"auto\"");

        assert_eq!(
            render_position(&MonitorPosition::AutoRight),
            "\"auto-right\""
        );

        assert_eq!(
            render_position(&MonitorPosition::Fixed { x: 2560, y: 0 }),
            "\"2560x0\""
        );
    }
}
