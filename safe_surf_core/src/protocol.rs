use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HandshakeInit {
    pub client_version: String,
    pub ephemeral_public_key: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HandshakeResponse {
    pub server_version: String,
    pub ephemeral_public_key: [u8; 32],
    pub salt: [u8; 16],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    HandshakeInit(HandshakeInit),
    HandshakeResponse(HandshakeResponse),
    EncryptedPayload {
        nonce: [u8; 24],
        ciphertext: Vec<u8>,
    },
    Error {
        code: u32,
        message: String,
    },
    PageContent(PageContent),
    RiskReport(RiskReport),
    Ping,
    Pong,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageContent {
    pub url: String,
    pub html: String,
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskReport {
    pub score: f32, // 0.0 to 1.0 (1.0 is high risk)
    pub findings: Vec<String>,
    pub recommended_action: String,
}
