use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoConfig {
    pub argon2_m_cost: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,
    pub session_rotation_bytes: u64,
    pub session_rotation_seconds: u64,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            argon2_m_cost: 65536, // 64MB
            argon2_t_cost: 3,
            argon2_p_cost: 4,
            session_rotation_bytes: 1024 * 1024 * 1024, // 1GB
            session_rotation_seconds: 3600, // 1 hour
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetyConfig {
    pub timing_jitter_ms: u32,
    pub cover_traffic_enabled: bool,
    pub sanitization_level: String, // "Strict", "Moderate", "VisualOnly"
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            timing_jitter_ms: 50,
            cover_traffic_enabled: false,
            sanitization_level: "Strict".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SafeSurfConfig {
    pub crypto: CryptoConfig,
    pub safety: SafetyConfig,
}
