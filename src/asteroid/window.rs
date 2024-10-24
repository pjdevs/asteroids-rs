use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};

pub fn asteroid_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Asteroid-rs".into(),
            name: Some("pjdevs.asteroid-rs".into()),
            resolution: WindowResolution::new(800.0, 800.0),
            present_mode: PresentMode::AutoVsync,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}
