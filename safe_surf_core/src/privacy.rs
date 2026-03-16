use crate::config::SafetyConfig;
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;

pub struct PrivacyGuard {
    config: SafetyConfig,
}

impl PrivacyGuard {
    pub fn new(config: SafetyConfig) -> Self {
        Self { config }
    }

    /// Adds a randomized delay to obfuscate timing metadata
    pub async fn apply_pacing(&self) {
        if self.config.timing_jitter_ms > 0 {
            let jitter = rand::thread_rng().gen_range(0..self.config.timing_jitter_ms);
            sleep(Duration::from_millis(jitter as u64)).await;
        }
    }

    /// Generates a cover (dummy) request profile
    /// This is a stub for a more complex traffic shaping engine
    pub fn get_cover_traffic_schedule(&self) -> Vec<Duration> {
        if !self.config.cover_traffic_enabled {
            return Vec::new();
        }
        
        let mut schedule = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            schedule.push(Duration::from_millis(rng.gen_range(500..5000)));
        }
        schedule
    }
}
