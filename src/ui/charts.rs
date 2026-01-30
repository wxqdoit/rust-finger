use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::HashMap;

/// Hourly activity chart component
pub struct HourlyChart {
    hourly_counts: HashMap<u8, u64>,
    max_count: u64,
}

impl HourlyChart {
    pub fn new(hourly_counts: HashMap<u8, u64>) -> Self {
        let max_count = hourly_counts.values().copied().max().unwrap_or(1);
        Self { hourly_counts, max_count }
    }
    
    fn render_bar(&self, hour: u8) -> impl IntoElement {
        let count = self.hourly_counts.get(&hour).copied().unwrap_or(0);
        let height_percent = if self.max_count > 0 {
            (count as f32 / self.max_count as f32 * 100.0).max(2.0)
        } else {
            2.0
        };
        
        // Current hour highlight
        let current_hour = chrono::Local::now().hour() as u8;
        let is_current = hour == current_hour;
        
        let bar_color = if is_current {
            rgb(0xff9e64) // Orange for current hour
        } else if count > 0 {
            rgb(0x7aa2f7) // Blue for activity
        } else {
            rgb(0x414868) // Gray for no activity
        };
        
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_end()
            .gap_1()
            .child(
                // Bar
                div()
                    .w_3()
                    .rounded_t_sm()
                    .bg(bar_color)
                    .h(relative(height_percent / 100.0))
                    .when(is_current, |this: Div| {
                        this.shadow_md()
                    })
            )
            .child(
                // Hour label
                div()
                    .text_xs()
                    .text_color(if is_current { rgb(0xff9e64) } else { rgb(0x565f89) })
                    .child(format!("{}", hour))
            )
    }
}

impl IntoElement for HourlyChart {
    type Element = Div;
    
    fn into_element(self) -> Self::Element {
        div()
            .flex_1()
            .flex()
            .gap_1()
            .pb_4()
            .children((0..24).map(|hour| self.render_bar(hour)))
    }
}

use chrono::Timelike;
