use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};

/// Statistics data that can be persisted
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    /// Key press counts per key name
    pub key_counts: HashMap<String, u64>,
    
    /// Mouse button click counts (left, right, middle, etc.)
    pub mouse_clicks: HashMap<String, u64>,
    
    /// Total mouse movement distance in pixels
    pub mouse_distance: f64,
    
    /// Total scroll distance
    pub scroll_distance: i64,
    
    /// Hourly statistics (hour 0-23 -> counts)
    pub hourly_key_counts: HashMap<u8, u64>,
    pub hourly_click_counts: HashMap<u8, u64>,
    
    /// Daily statistics
    pub daily_stats: HashMap<String, DailyStats>,
    
    /// Session start time
    #[serde(skip)]
    pub session_start: Option<Instant>,
    
    /// Keys pressed in current minute (for WPM calculation)
    #[serde(skip)]
    pub recent_keys: Vec<Instant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DailyStats {
    pub total_keys: u64,
    pub total_clicks: u64,
    pub total_distance: f64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            session_start: Some(Instant::now()),
            ..Default::default()
        }
    }
    
    /// Record a key press event
    pub fn record_key(&mut self, key_name: String) {
        // Update key count
        *self.key_counts.entry(key_name).or_insert(0) += 1;
        
        // Update hourly stats
        let hour = Local::now().hour() as u8;
        *self.hourly_key_counts.entry(hour).or_insert(0) += 1;
        
        // Update daily stats
        let date = Local::now().format("%Y-%m-%d").to_string();
        self.daily_stats
            .entry(date)
            .or_insert_with(DailyStats::default)
            .total_keys += 1;
        
        // Track recent keys for WPM
        let now = Instant::now();
        self.recent_keys.retain(|t| now.duration_since(*t) < Duration::from_secs(60));
        self.recent_keys.push(now);
    }
    
    /// Record a mouse click event
    pub fn record_click(&mut self, button: String) {
        *self.mouse_clicks.entry(button).or_insert(0) += 1;
        
        let hour = Local::now().hour() as u8;
        *self.hourly_click_counts.entry(hour).or_insert(0) += 1;
        
        let date = Local::now().format("%Y-%m-%d").to_string();
        self.daily_stats
            .entry(date)
            .or_insert_with(DailyStats::default)
            .total_clicks += 1;
    }
    
    /// Record mouse movement
    pub fn record_movement(&mut self, distance: f64) {
        self.mouse_distance += distance;
        
        let date = Local::now().format("%Y-%m-%d").to_string();
        self.daily_stats
            .entry(date)
            .or_insert_with(DailyStats::default)
            .total_distance += distance;
    }
    
    /// Record scroll event
    pub fn record_scroll(&mut self, delta: i64) {
        self.scroll_distance += delta.abs();
    }
    
    /// Calculate current typing speed (words per minute)
    /// Assumes average word length of 5 characters
    pub fn current_wpm(&self) -> f64 {
        let now = Instant::now();
        let keys_in_minute: usize = self.recent_keys
            .iter()
            .filter(|t| now.duration_since(**t) < Duration::from_secs(60))
            .count();
        
        // Characters per minute / 5 = WPM
        keys_in_minute as f64 / 5.0
    }
    
    /// Get total key presses for today
    pub fn today_keys(&self) -> u64 {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.daily_stats
            .get(&today)
            .map(|s| s.total_keys)
            .unwrap_or(0)
    }
    
    /// Get total clicks for today
    pub fn today_clicks(&self) -> u64 {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.daily_stats
            .get(&today)
            .map(|s| s.total_clicks)
            .unwrap_or(0)
    }
    
    /// Get total mouse distance for today
    pub fn today_distance(&self) -> f64 {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.daily_stats
            .get(&today)
            .map(|s| s.total_distance)
            .unwrap_or(0.0)
    }
    
    /// Get top N most pressed keys
    pub fn top_keys(&self, n: usize) -> Vec<(String, u64)> {
        let mut sorted: Vec<_> = self.key_counts.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(n);
        sorted
    }
    
    /// Get session duration
    pub fn session_duration(&self) -> Duration {
        self.session_start
            .map(|start| start.elapsed())
            .unwrap_or_default()
    }
}

/// Thread-safe statistics manager
#[derive(Clone)]
pub struct StatsManager {
    stats: Arc<RwLock<Stats>>,
    data_path: PathBuf,
    pub listener_active: Arc<AtomicBool>,
    pub last_error: Arc<RwLock<Option<String>>>,
    // Deduplication state
    last_key: Arc<RwLock<Option<(String, Instant)>>>,
    last_click: Arc<RwLock<Option<(String, Instant)>>>,
}

impl StatsManager {
    pub fn new() -> Self {
        let data_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rust-finger")
            .join("stats.json");
        
        // Ensure directory exists
        if let Some(parent) = data_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Load existing stats or create new
        let stats = Self::load_from_file(&data_path).unwrap_or_else(|_| Stats::new());
        
        Self {
            stats: Arc::new(RwLock::new(stats)),
            data_path,
            listener_active: Arc::new(AtomicBool::new(false)),
            last_error: Arc::new(RwLock::new(None)),
            last_key: Arc::new(RwLock::new(None)),
            last_click: Arc::new(RwLock::new(None)),
        }
    }
    
    pub fn set_listener_active(&self, active: bool) {
        self.listener_active.store(active, Ordering::SeqCst);
    }
    
    pub fn set_listener_error(&self, error: String) {
        if let Ok(mut lock) = self.last_error.write() {
            *lock = Some(error);
        }
    }
    
    pub fn is_listener_active(&self) -> bool {
        self.listener_active.load(Ordering::SeqCst)
    }
    
    pub fn get_listener_error(&self) -> Option<String> {
        self.last_error.read().ok()?.clone()
    }
    
    fn load_from_file(path: &PathBuf) -> Result<Stats, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut stats: Stats = serde_json::from_str(&content)?;
        stats.session_start = Some(Instant::now());
        Ok(stats)
    }
    
    /// Save stats to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stats = self.stats.read().map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&*stats)?;
        fs::write(&self.data_path, json)?;
        Ok(())
    }
    
    /// Record a key press with deduplication
    pub fn record_key(&self, key_name: String) {
        // Simple deduplication (50ms window)
        let now = Instant::now();
        if let Ok(mut last) = self.last_key.write() {
            if let Some((last_name, last_time)) = &*last {
                if last_name == &key_name && now.duration_since(*last_time) < Duration::from_millis(50) {
                    return;
                }
            }
            *last = Some((key_name.clone(), now));
        }
        
        if let Ok(mut stats) = self.stats.write() {
            stats.record_key(key_name);
        }
    }
    
    /// Record a mouse click with deduplication
    pub fn record_click(&self, button: String) {
        // Simple deduplication (50ms window)
        let now = Instant::now();
        if let Ok(mut last) = self.last_click.write() {
            if let Some((last_name, last_time)) = &*last {
                if last_name == &button && now.duration_since(*last_time) < Duration::from_millis(50) {
                    return;
                }
            }
            *last = Some((button.clone(), now));
        }
        
        if let Ok(mut stats) = self.stats.write() {
            stats.record_click(button);
        }
    }
    
    /// Record mouse movement
    pub fn record_movement(&self, distance: f64) {
        if let Ok(mut stats) = self.stats.write() {
            stats.record_movement(distance);
        }
    }
    
    /// Record scroll
    pub fn record_scroll(&self, delta: i64) {
        if let Ok(mut stats) = self.stats.write() {
            stats.record_scroll(delta);
        }
    }
    
    /// Get a snapshot of current stats
    pub fn snapshot(&self) -> Stats {
        self.stats.read()
            .map(|s| s.clone())
            .unwrap_or_default()
    }
}

use chrono::Timelike;
