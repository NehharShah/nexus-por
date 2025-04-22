use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Participant {
    pub id: String, // Could be address, pubkey, etc.
    pub strikes: u8,
    pub reputation: i32,
    pub blacklisted: bool,
    pub blacklist_reason: Option<String>,
    pub last_action: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PolicyConfig {
    pub max_strikes: u8,
    pub blacklist_threshold: u8,
    pub penalty_fee: u64,
    pub suspension_duration: u64,
    pub reputation_penalty: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionLog {
    pub participant_id: String,
    pub action: String,
    pub timestamp: u64,
    pub details: Option<String>,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            max_strikes: 3,
            blacklist_threshold: 5,
            penalty_fee: 1000,
            suspension_duration: 3600, // seconds
            reputation_penalty: -10,
        }
    }
}
