/// AutoSave functionality for Marco 2.0
/// 
/// Handles automatic project saving and recovery

use std::time::Instant;

pub struct AutoSave {
    last_save: Instant,
    save_interval: std::time::Duration,
}

impl AutoSave {
    pub fn new() -> Self {
        Self {
            last_save: Instant::now(),
            save_interval: std::time::Duration::from_secs(300), // 5 minutes
        }
    }

    pub fn should_save(&self) -> bool {
        self.last_save.elapsed() >= self.save_interval
    }

    pub fn mark_saved(&mut self) {
        self.last_save = Instant::now();
    }
}
