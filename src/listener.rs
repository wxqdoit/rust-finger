use rdev::{listen, Button, Event, EventType, Key};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::stats::StatsManager;

/// Input event types for communication
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(String),
    KeyRelease(String),
    MouseClick(String),
    MouseMove { x: f64, y: f64 },
    Scroll { delta_x: i64, delta_y: i64 },
}

/// Global input listener that runs in a separate thread
pub struct InputListener {
    stats: StatsManager,
    last_mouse_pos: Option<(f64, f64)>,
}

impl InputListener {
    pub fn new(stats: StatsManager) -> Self {
        Self {
            stats,
            last_mouse_pos: None,
        }
    }
    
    /// Start listening for global input events
    /// This function will block - run it in a separate thread
    pub fn start(stats: StatsManager) {
        let stats_clone = stats.clone();
        
        thread::spawn(move || {
            let mut last_pos: Option<(f64, f64)> = None;
            let callback_stats = stats_clone.clone();
            
            let callback = move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        let key_name = key_to_string(&key);
                        callback_stats.record_key(key_name);
                    }
                    EventType::KeyRelease(_) => {
                        // We only count key presses, not releases
                    }
                    EventType::ButtonPress(button) => {
                        let button_name = button_to_string(&button);
                        callback_stats.record_click(button_name);
                    }
                    EventType::ButtonRelease(_) => {
                        // We only count button presses
                    }
                    EventType::MouseMove { x, y } => {
                        if let Some((last_x, last_y)) = last_pos {
                            let dx = x - last_x;
                            let dy = y - last_y;
                            let distance = (dx * dx + dy * dy).sqrt();
                            callback_stats.record_movement(distance);
                        }
                        last_pos = Some((x, y));
                    }
                    EventType::Wheel { delta_x, delta_y } => {
                        callback_stats.record_scroll(delta_y);
                    }
                }
            };
            
            log::info!("Starting global input listener...");
            stats_clone.set_listener_active(true);
            
            if let Err(error) = listen(callback) {
                stats_clone.set_listener_active(false);
                stats_clone.set_listener_error(format!("{:?}", error));
                log::error!("Error in input listener: {:?}", error);
            }
        });
    }
}

/// Convert rdev Key to a human-readable string
fn key_to_string(key: &Key) -> String {
    match key {
        // Letters
        Key::KeyA => "A".to_string(),
        Key::KeyB => "B".to_string(),
        Key::KeyC => "C".to_string(),
        Key::KeyD => "D".to_string(),
        Key::KeyE => "E".to_string(),
        Key::KeyF => "F".to_string(),
        Key::KeyG => "G".to_string(),
        Key::KeyH => "H".to_string(),
        Key::KeyI => "I".to_string(),
        Key::KeyJ => "J".to_string(),
        Key::KeyK => "K".to_string(),
        Key::KeyL => "L".to_string(),
        Key::KeyM => "M".to_string(),
        Key::KeyN => "N".to_string(),
        Key::KeyO => "O".to_string(),
        Key::KeyP => "P".to_string(),
        Key::KeyQ => "Q".to_string(),
        Key::KeyR => "R".to_string(),
        Key::KeyS => "S".to_string(),
        Key::KeyT => "T".to_string(),
        Key::KeyU => "U".to_string(),
        Key::KeyV => "V".to_string(),
        Key::KeyW => "W".to_string(),
        Key::KeyX => "X".to_string(),
        Key::KeyY => "Y".to_string(),
        Key::KeyZ => "Z".to_string(),
        
        // Numbers
        Key::Num0 => "0".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        
        // Function keys
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        
        // Modifiers
        Key::ShiftLeft => "Shift".to_string(),
        Key::ShiftRight => "Shift".to_string(),
        Key::ControlLeft => "Ctrl".to_string(),
        Key::ControlRight => "Ctrl".to_string(),
        Key::Alt => "Alt".to_string(),
        Key::AltGr => "AltGr".to_string(),
        Key::MetaLeft => "Meta".to_string(),
        Key::MetaRight => "Meta".to_string(),
        
        // Special keys
        Key::Space => "Space".to_string(),
        Key::Return => "Enter".to_string(),
        Key::Escape => "Esc".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::CapsLock => "CapsLock".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        
        // Arrow keys
        Key::UpArrow => "↑".to_string(),
        Key::DownArrow => "↓".to_string(),
        Key::LeftArrow => "←".to_string(),
        Key::RightArrow => "→".to_string(),
        
        // Punctuation
        Key::Comma => ",".to_string(),
        Key::Dot => ".".to_string(),
        Key::Slash => "/".to_string(),
        Key::SemiColon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::BackSlash => "\\".to_string(),
        Key::LeftBracket => "[".to_string(),
        Key::RightBracket => "]".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::BackQuote => "`".to_string(),
        
        // Unknown
        Key::Unknown(code) => format!("Key({})", code),
        
        // Any other key
        _ => format!("{:?}", key),
    }
}

/// Convert rdev Button to a human-readable string
fn button_to_string(button: &Button) -> String {
    match button {
        Button::Left => "Left".to_string(),
        Button::Right => "Right".to_string(),
        Button::Middle => "Middle".to_string(),
        Button::Unknown(code) => format!("Button({})", code),
    }
}
