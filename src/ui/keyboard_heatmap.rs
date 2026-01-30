use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::HashMap;

/// Keyboard layout for QWERTY
const KEYBOARD_ROWS: &[&[&str]] = &[
    &["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "Backspace"],
    &["Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\"],
    &["CapsLock", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "Enter"],
    &["Shift", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Shift"],
    &["Ctrl", "Meta", "Alt", "Space", "Alt", "Meta", "Ctrl"],
];

/// Key widths in units (1 unit = standard key width)
fn get_key_width(key: &str) -> f32 {
    match key {
        "Backspace" => 2.0,
        "Tab" => 1.5,
        "\\" => 1.5,
        "CapsLock" => 1.75,
        "Enter" => 2.25,
        "Shift" => 2.25,
        "Ctrl" | "Meta" | "Alt" => 1.25,
        "Space" => 6.25,
        _ => 1.0,
    }
}

/// Keyboard heatmap component with realistic key styling
pub struct KeyboardHeatmap {
    key_counts: HashMap<String, u64>,
    max_count: u64,
}

impl KeyboardHeatmap {
    pub fn new(key_counts: HashMap<String, u64>) -> Self {
        let max_count = key_counts.values().copied().max().unwrap_or(1);
        Self { key_counts, max_count }
    }
    
    /// Get heat color based on key usage intensity
    fn heat_color(&self, key: &str) -> (Rgba, Rgba, Rgba) {
        let count = self.key_counts.get(key).copied().unwrap_or(0);
        let intensity = if self.max_count > 0 {
            (count as f32 / self.max_count as f32).min(1.0)
        } else {
            0.0
        };
        
        // Returns (top_color, face_color, shadow_color)
        if intensity < 0.01 {
            // Not used - dark gray with 3D effect
            (rgb(0x3a3a4a), rgb(0x2a2a3a), rgb(0x1a1a2a))
        } else if intensity < 0.25 {
            // Low usage - blue
            (rgb(0x5b7bb8), rgb(0x4a6aa8), rgb(0x3a5a98))
        } else if intensity < 0.5 {
            // Medium usage - cyan/teal
            (rgb(0x5bc8b8), rgb(0x4ab8a8), rgb(0x3aa898))
        } else if intensity < 0.75 {
            // High usage - yellow/amber
            (rgb(0xf0c060), rgb(0xe0b050), rgb(0xd0a040))
        } else {
            // Very high usage - orange/red
            (rgb(0xf08060), rgb(0xe07050), rgb(0xd06040))
        }
    }
    
    fn render_key(&self, key: &str) -> impl IntoElement {
        let width = get_key_width(key);
        let count = self.key_counts.get(key).copied().unwrap_or(0);
        let (top_color, face_color, _shadow_color) = self.heat_color(key);
        
        let display_key = match key {
            "Backspace" => "⌫",
            "Tab" => "Tab",
            "CapsLock" => "Caps",
            "Enter" => "Enter",
            "Shift" => "Shift",
            "Ctrl" => "Ctrl",
            "Meta" => "Win",
            "Alt" => "Alt",
            "Space" => "",
            _ => key,
        };
        
        let key_width = px(width * 38.0);
        let key_height = px(36.0);
        
        // Outer container with shadow
        div()
            .w(key_width)
            .h(key_height)
            .m(px(2.0))
            .rounded_md()
            .bg(rgb(0x0a0a10)) // Deep shadow base
            .shadow_md()
            // Inner key with 3D effect
            .child(
                div()
                    .w_full()
                    .h_full()
                    .rounded_md()
                    .bg(face_color)
                    .border_1()
                    .border_color(rgba(0xffffff20))
                    .relative()
                    // Top highlight edge
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .right_0()
                            .h(px(2.0))
                            .bg(top_color)
                            .rounded_t_md()
                    )
                    // Key face content
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .pt(px(2.0))
                            // Key label
                            .child(
                                div()
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(display_key.to_string())
                            )
                            // Count display
                            .when(count > 0, |this: Div| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .font_family("JetBrains Mono")
                                        .text_color(rgba(0xffffffcc))
                                        .child(if count > 999 {
                                            format!("{}k", count / 1000)
                                        } else {
                                            format!("{}", count)
                                        })
                                )
                            })
                    )
                    .hover(|s| s.border_color(rgb(0x7aa2f7)).shadow_lg())
            )
    }
}

impl IntoElement for KeyboardHeatmap {
    type Element = Div;
    
    fn into_element(self) -> Self::Element {
        // Keyboard base with realistic styling
        div()
            .p_3()
            .bg(rgb(0x1a1a24))
            .rounded_xl()
            .border_1()
            .border_color(rgb(0x2a2a3a))
            .shadow_lg()
            // Inner keyboard plate
            .child(
                div()
                    .p_2()
                    .bg(rgb(0x12121a))
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(0x252530))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_px()
                    .children(KEYBOARD_ROWS.iter().map(|row| {
                        div()
                            .flex()
                            .justify_center()
                            .children(row.iter().map(|key| self.render_key(key)))
                    }))
            )
    }
}
