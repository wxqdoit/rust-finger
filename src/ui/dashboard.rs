use gpui::*;
use crate::stats::{Stats, StatsManager};
use super::keyboard_heatmap::KeyboardHeatmap;
use super::charts::HourlyChart;
use std::time::Duration;

/// Main dashboard view showing all statistics
pub struct Dashboard {
    stats_manager: StatsManager,
    stats_snapshot: Stats,
    focus_handle: FocusHandle,
    main_scroll: ScrollHandle,
    top_scroll: ScrollHandle,
}

impl Dashboard {
    pub fn new(cx: &mut Context<Self>, stats_manager: StatsManager) -> Self {
        let stats_snapshot = stats_manager.snapshot();
        let focus_handle = cx.focus_handle();
        Self {
            stats_manager,
            stats_snapshot,
            focus_handle,
            main_scroll: ScrollHandle::new(),
            top_scroll: ScrollHandle::new(),
        }
    }
    
    /// Refresh statistics snapshot
    pub fn refresh(&mut self) {
        self.stats_snapshot = self.stats_manager.snapshot();
    }
}

impl Render for Dashboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Refresh stats
        self.refresh();
        
        // Schedule next refresh (100ms) - real-time updates
        cx.spawn_in(window, async move |this, mut cx| {
            cx.background_executor().timer(Duration::from_millis(100)).await;
            let _ = this.update(cx, |dashboard, cx| {
                dashboard.refresh();
                cx.notify();
            });
        }).detach();

        
        let stats = &self.stats_snapshot;
        let today_keys = stats.today_keys();
        let today_clicks = stats.today_clicks();
        let today_distance = stats.today_distance();
        let wpm = stats.current_wpm();
        let session = stats.session_duration();
        let total_keys: u64 = stats.key_counts.values().sum();
        let total_clicks: u64 = stats.mouse_clicks.values().sum();
        let top_keys = stats.top_keys(20);
        
        // Wrap everything in a relative container to position resize handles
        let stats_manager = self.stats_manager.clone();
        
        div()
            .relative()
            .size_full()
            .track_focus(&self.focus_handle) // Use tracked focus handle
            .on_key_down(move |event, _window, _cx| {
                let keystroke = &event.keystroke;
                let key = if keystroke.key.len() == 1 {
                    keystroke.key.to_uppercase()
                } else {
                    // Capitalize first letter for special keys
                    let mut c = keystroke.key.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                };
                stats_manager.record_key(key);
            })
            .on_mouse_down(MouseButton::Left, {
                let stats_manager = self.stats_manager.clone();
                move |_event, _window, _cx| {
                    stats_manager.record_click("Left".to_string());
                }
            })
            .on_mouse_down(MouseButton::Right, {
                let stats_manager = self.stats_manager.clone();
                move |_event, _window, _cx| {
                    stats_manager.record_click("Right".to_string());
                }
            })
            .child(
                div()
                    .id("main-container")
                    .size_full()
                    .bg(rgb(0x0f0f14))
                    .text_color(rgb(0xe0e0e0))
                    .font_family("JetBrains Mono")
                    .flex()
                    .flex_col()
                    // Menu Bar (Draggable)
                    .child(
                        div()
                            .id("menu-bar")
                            .w_full()
                            .h_10()
                            .bg(rgb(0x16161e))
                            .border_b_1()
                            .border_color(rgb(0x2a2a3a))
                            .flex()
                            .items_center()
                            .px_4()
                            .gap_4()
                            // Add window drag handler
                            .on_mouse_down(MouseButton::Left, move |_ev, window, _cx| {
                                window.start_window_move();
                            })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x7aa2f7))
                                            .child("⌨️ Finger Monitor")
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x565f89))
                                            .px_2()
                                            .py_1()
                                            .bg(rgb(0x1a1b26))
                                            .rounded_md()
                                            .child("v0.1.0")
                                    )
                            )
                            .child(div().flex_1())
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_xs().text_color(rgb(0x565f89)).child("Session"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x9ece6a))
                                            .child(format!("{:02}:{:02}:{:02}",
                                                session.as_secs() / 3600,
                                                (session.as_secs() % 3600) / 60,
                                                session.as_secs() % 60
                                            ))
                                    )
                            )
                            .child({
                                let is_active = self.stats_manager.is_listener_active();
                                let (color, text) = if is_active {
                                    (rgb(0x73daca), "LIVE")
                                } else {
                                    (rgb(0xf7768e), "OFFLINE")
                                };
                                
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(div().w_2().h_2().rounded_full().bg(color))
                                    .child(div().text_xs().text_color(color).child(text))
                            })
                            // Window control buttons (simple style)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .ml_3()
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()) // Prevent dragging when clicking buttons
                                    // Minimize button
                                    .child(
                                        div()
                                            .id("btn-minimize")
                                            .w_7()
                                            .h_7()
                                            .rounded_md()
                                            .bg(rgb(0x2a2a3a))
                                            .border_1()
                                            .border_color(rgb(0x3a3a4a))
                                            .hover(|s| s.bg(rgb(0x3a3a4a)).border_color(rgb(0x4a4a5a)))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(rgb(0x888898))
                                            .child("—")
                                            .on_click(move |_ev, window, _cx| {
                                                window.minimize_window();
                                            })
                                    )
                                    // Maximize button
                                    .child(
                                        div()
                                            .id("btn-maximize")
                                            .w_7()
                                            .h_7()
                                            .rounded_md()
                                            .bg(rgb(0x2a2a3a))
                                            .border_1()
                                            .border_color(rgb(0x3a3a4a))
                                            .hover(|s| s.bg(rgb(0x3a3a4a)).border_color(rgb(0x4a4a5a)))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(rgb(0x888898))
                                            .child("☐")
                                            .on_click(move |_ev, window, _cx| {
                                                window.toggle_fullscreen();
                                            })
                                    )
                                    // Close button
                                    .child(
                                        div()
                                            .id("btn-close")
                                            .w_7()
                                            .h_7()
                                            .rounded_md()
                                            .bg(rgb(0x2a2a3a))
                                            .border_1()
                                            .border_color(rgb(0x3a3a4a))
                                            .hover(|s| s.bg(rgb(0x5a2a2a)).border_color(rgb(0x7a3a3a)).text_color(rgb(0xff6666)))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(rgb(0x888898))
                                            .child("✕")
                                            .on_click(move |_ev, _window, cx| {
                                                cx.quit();
                                            })
                                    )
                            )
                    )
                    // Main scrollable content
                    .child(
                        div()
                            .flex_1()
                            .relative()
                            .child(
                                div()
                                    .id("content-area")
                                    .absolute()
                                    .size_full()
                                    .track_scroll(&self.main_scroll)
                                    .overflow_y_scroll()
                                    .overflow_x_hidden()
                                    .p_4()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    // Stats cards row
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .flex_wrap()
                                            .child(self.render_stat_card("Today Keys", &format!("{}", today_keys), "⌨️", rgb(0x7aa2f7).into()))
                                            .child(self.render_stat_card("Today Clicks", &format!("{}", today_clicks), "🖱️", rgb(0xbb9af7).into()))
                                            .child(self.render_stat_card("Distance", &format!("{:.2} m", today_distance / 1000.0), "📏", rgb(0x9ece6a).into()))
                                            .child(self.render_stat_card("WPM", &format!("{:.0}", wpm), "⚡", rgb(0xff9e64).into()))
                                    )
                                    // Second row - All time stats
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .child(self.render_stat_card_small("All-time Keys", &format!("{}", total_keys), rgb(0x7aa2f7).into()))
                                            .child(self.render_stat_card_small("All-time Clicks", &format!("{}", total_clicks), rgb(0xbb9af7).into()))
                                            .child(self.render_stat_card_small("Total Distance", &format!("{:.2} km", stats.mouse_distance / 1_000_000.0), rgb(0x9ece6a).into()))
                                            .child(self.render_stat_card_small("Scroll", &format!("{}", stats.scroll_distance), rgb(0xe0af68).into()))
                                    )
                                    // Main content row
                                    .child(
                                        div()
                                            .flex()
                                            .gap_4()
                                            .min_h_80()
                                            // Keyboard heatmap
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .rounded_xl()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_base()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .mb_3()
                                                            .child("🌡️ Keyboard Heatmap")
                                                    )
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .child(KeyboardHeatmap::new(stats.key_counts.clone()))
                                                    )
                                            )
                                            // Top keys sidebar with scroll
                                            .child(
                                                div()
                                                    .w_64()
                                                    .bg(rgb(0x1a1b26))
                                                    .rounded_xl()
                                                    .p_4()
                                                    .border_1()
                                                    .border_color(rgb(0x2a2a3a))
                                                    .flex()
                                                    .flex_col()
                                                    .max_h_full()
                                                    .child(
                                                        div()
                                                            .text_base()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .mb_3()
                                                            .flex()
                                                            .items_center()
                                                            .justify_between()
                                                            .child("🔥 Top Keys")
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x565f89))
                                                                    .child(format!("({})", top_keys.len()))
                                                            )
                                                    )
                                                    // Scrollable keys list with scrollbar
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .relative()
                                                            .child(
                                                                div()
                                                                    .id("top-keys-scroll")
                                                                    .absolute()
                                                                    .size_full()
                                                                    .track_scroll(&self.top_scroll)
                                                                    .overflow_y_scroll()
                                                                    .overflow_x_hidden()
                                                                    .children(
                                                                        top_keys.iter().enumerate().map(|(i, (key, count))| {
                                                                            self.render_top_key_item(i + 1, key, *count)
                                                                        })
                                                                    )
                                                            )
                                                            .child(self.render_scrollbar(&self.top_scroll))
                                                    )
                                            )
                                    )
                                    // Mouse stats row
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .child(self.render_mouse_card("Left Click", stats.mouse_clicks.get("Left").copied().unwrap_or(0), rgb(0x7aa2f7)))
                                            .child(self.render_mouse_card("Right Click", stats.mouse_clicks.get("Right").copied().unwrap_or(0), rgb(0xbb9af7)))
                                            .child(self.render_mouse_card("Middle Click", stats.mouse_clicks.get("Middle").copied().unwrap_or(0), rgb(0x9ece6a)))
                                    )
                                    // Hourly chart
                                    .child(
                                        div()
                                            .h_40()
                                            .bg(rgb(0x1a1b26))
                                            .rounded_xl()
                                            .p_4()
                                            .border_1()
                                            .border_color(rgb(0x2a2a3a))
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .mb_2()
                                                    .child("📊 Today's Activity")
                                            )
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .child(HourlyChart::new(stats.hourly_key_counts.clone()))
                                            )
                                    )
                            )
                            .child(self.render_scrollbar(&self.main_scroll))
                    )
                    // Status Bar
                    .child(
                        div()
                            .id("status-bar")
                            .w_full()
                            .h_7()
                            .bg(rgb(0x16161e))
                            .border_t_1()
                            .border_color(rgb(0x2a2a3a))
                            .flex()
                            .items_center()
                            .px_4()
                            .gap_6()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(div().text_xs().text_color(rgb(0x565f89)).child("Total:"))
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(rgb(0x7aa2f7)).child(format!("{} keys", total_keys)))
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(rgb(0xbb9af7)).child(format!("{} clicks", total_clicks)))
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(div().text_xs().text_color(rgb(0x565f89)).child("WPM:"))
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(rgb(0xff9e64)).child(format!("{:.0}", wpm)))
                            )
                            .child(div().flex_1())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x565f89))
                                    .child("~/.local/share/rust-finger")
                            )
                    )
            )
            // Resize Handles
            // Top
            .child(self.render_resize_handle(ResizeEdge::Top, 4.0, true))
            // Bottom
            .child(self.render_resize_handle(ResizeEdge::Bottom, 4.0, true))
            // Left
            .child(self.render_resize_handle(ResizeEdge::Left, 4.0, false))
            // Right
            .child(self.render_resize_handle(ResizeEdge::Right, 4.0, false))
            // TopLeft
            .child(self.render_resize_corner(ResizeEdge::TopLeft))
            // TopRight
            .child(self.render_resize_corner(ResizeEdge::TopRight))
            // BottomLeft
            .child(self.render_resize_corner(ResizeEdge::BottomLeft))
            // BottomRight
            .child(self.render_resize_corner(ResizeEdge::BottomRight))
    }
}

impl Dashboard {
    fn render_resize_handle(&self, edge: ResizeEdge, size: f32, horizontal: bool) -> Div {
        let mut div = div()
            .absolute()
            .bg(rgba(0x00000000)); // Transparent
            
        div = match edge {
            ResizeEdge::Top => div.top_0().left_0().right_0().h(px(size)).cursor_row_resize(),
            ResizeEdge::Bottom => div.bottom_0().left_0().right_0().h(px(size)).cursor_row_resize(),
            ResizeEdge::Left => div.left_0().top_0().bottom_0().w(px(size)).cursor_col_resize(),
            ResizeEdge::Right => div.right_0().top_0().bottom_0().w(px(size)).cursor_col_resize(),
            _ => div,
        };
        
        div.on_mouse_down(MouseButton::Left, move |_ev, window, _cx| {
            window.start_window_resize(edge);
        })
    }
    
    fn render_resize_corner(&self, edge: ResizeEdge) -> Div {
        let size = px(8.0);
        let mut div = div()
            .absolute()
            .w(size)
            .h(size)
            .bg(rgba(0x00000000)); // Transparent
            
        div = match edge {
            ResizeEdge::TopLeft => div.top_0().left_0().cursor_nwse_resize(),
            ResizeEdge::TopRight => div.top_0().right_0().cursor_nesw_resize(),
            ResizeEdge::BottomLeft => div.bottom_0().left_0().cursor_nesw_resize(),
            ResizeEdge::BottomRight => div.bottom_0().right_0().cursor_nwse_resize(),
            _ => div,
        };
        
        div.on_mouse_down(MouseButton::Left, move |_ev, window, _cx| {
            window.start_window_resize(edge);
        })
    }

    fn render_stat_card(&self, label: &str, value: &str, icon: &str, accent_color: Hsla) -> Div {
        div()
            .flex_1()
            .min_w_40()
            .bg(rgb(0x1a1b26))
            .rounded_xl()
            .p_4()
            .border_1()
            .border_color(rgb(0x2a2a3a))
            .shadow_sm()
            .hover(|s| s.border_color(accent_color).bg(rgb(0x1f2030)).shadow_md())
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_lg().child(icon.to_string()))
                    .child(div().text_xs().text_color(rgb(0x565f89)).child(label.to_string()))
            )
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(accent_color)
                    .child(value.to_string())
            )
    }
    
    fn render_stat_card_small(&self, label: &str, value: &str, accent_color: Hsla) -> Div {
        div()
            .flex_1()
            .bg(rgb(0x1a1b26))
            .rounded_lg()
            .p_3()
            .border_1()
            .border_color(rgb(0x2a2a3a))
            .shadow_sm()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x565f89))
                    .child(label.to_string())
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(accent_color)
                    .child(value.to_string())
            )
    }
    
    fn render_top_key_item(&self, rank: usize, key: &str, count: u64) -> Div {
        let rank_color = match rank {
            1 => rgb(0xffd700),
            2 => rgb(0xc0c0c0),
            3 => rgb(0xcd7f32),
            _ => rgb(0x565f89),
        };
        
        div()
            .flex()
            .items_center()
            .gap_2()
            .py_1()
            .px_2()
            .rounded_md()
            .hover(|s| s.bg(rgb(0x292e42)))
            .child(
                div()
                    .w_5()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rank_color)
                    .child(format!("{}", rank))
            )
            .child(
                div()
                    .px_2()
                    .py_px()
                    .bg(rgb(0x24283b))
                    .rounded_sm()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .min_w_8()
                    .text_center()
                    .child(key.to_string())
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x7aa2f7))
                    .child(format!("{}", count))
            )
    }
    
    fn render_mouse_card(&self, label: &str, count: u64, color: Rgba) -> Div {
        div()
            .flex_1()
            .bg(rgb(0x1a1b26))
            .rounded_xl()
            .p_4()
            .border_1()
            .border_color(rgb(0x2a2a3a))
            .hover(|s| s.border_color(color))
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(color)
                    .child(format!("{}", count))
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x565f89))
                    .child(label.to_string())
            )
    }

    
    fn render_scrollbar(&self, _handle: &ScrollHandle) -> Div {
        // Simple scrollbar track indicator
        // Note: GPUI's flex_grow() doesn't take percentage arguments,
        // so we show a static scrollbar indicator
        div()
            .absolute()
            .top_0()
            .right_0()
            .bottom_0()
            .w_2()
            .bg(rgb(0x1a1b26)) // Dark track
            .rounded_full()
            .child(
                div()
                    .w_full()
                    .h_8() // Fixed height thumb
                    .mt_2()
                    .bg(rgb(0x3b3b4f))
                    .rounded_full()
                    .hover(|s| s.bg(rgb(0x565f89)))
            )
    }
}

