mod listener;
mod stats;
mod ui;

use listener::InputListener;
use stats::StatsManager;

use std::thread;
use std::time::Duration;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();
    
    log::info!("Starting Finger Monitor...");
    
    // Create stats manager
    let stats_manager = StatsManager::new();
    
    // Start input listener in background thread
    InputListener::start(stats_manager.clone());
    
    // Set up periodic save
    let save_manager = stats_manager.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            if let Err(e) = save_manager.save() {
                log::error!("Failed to save stats: {}", e);
            } else {
                log::debug!("Stats saved successfully");
            }
        }
    });
    
    // Save stats on exit
    let exit_manager = stats_manager.clone();
    ctrlc::set_handler(move || {
        log::info!("Shutting down, saving stats...");
        let _ = exit_manager.save();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    
    // Run GPUI application (blocks until window closes)
    ui::app::run(stats_manager.clone());
    
    // Save before exit
    log::info!("Saving final stats...");
    let _ = stats_manager.save();
}
