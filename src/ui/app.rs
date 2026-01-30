use gpui::*;
use crate::stats::StatsManager;
use super::dashboard::Dashboard;

/// Run the GPUI application
pub fn run(stats_manager: StatsManager) {
    Application::new().run(move |cx: &mut App| {
        // Set up window options
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1200.0), px(800.0)),
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some("Finger Monitor".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            app_id: Some("finger-monitor".to_string()),
            window_background: WindowBackgroundAppearance::Opaque,
            window_min_size: Some(size(px(800.0), px(600.0))),
            ..Default::default()
        };
        
        // Open main window
        cx.open_window(window_options, |_window, cx| {
            cx.new(|cx| Dashboard::new(cx, stats_manager.clone()))
        }).expect("Failed to open window");
    });
}
