use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::collections::BTreeMap;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    pub id: String,
    pub strikes: u8,
    pub reputation: i32,
    pub blacklisted: bool,
    pub blacklist_reason: Option<String>,
    pub last_action: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyConfig {
    pub max_strikes: u8,
    pub blacklist_threshold: u8,
    pub penalty_fee: u64,
    pub suspension_duration: u64,
    pub reputation_penalty: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionLog {
    pub participant_id: String,
    pub action: String,
    pub timestamp: u64,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Appeal {
    pub participant_id: String,
    pub reason: String,
    pub timestamp: i64,
    pub reviewed: bool,
    pub approved: bool,
}

pub fn load_policy(path: &str) -> io::Result<PolicyConfig> {
    let content = fs::read_to_string(path)?;
    let config: PolicyConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn load_participants(path: &str) -> io::Result<BTreeMap<String, Participant>> {
    let content = fs::read_to_string(path)?;
    let list: Vec<Participant> = serde_json::from_str(&content)?;
    Ok(list.into_iter().map(|p| (p.id.clone(), p)).collect())
}

pub fn save_participants(path: &str, map: &BTreeMap<String, Participant>) -> io::Result<()> {
    let list: Vec<_> = map.values().cloned().collect();
    let json = serde_json::to_string_pretty(&list)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_logs(path: &str) -> io::Result<Vec<ActionLog>> {
    let content = fs::read_to_string(path)?;
    let logs: Vec<ActionLog> = serde_json::from_str(&content)?;
    Ok(logs)
}

pub fn save_logs(path: &str, logs: &[ActionLog]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(logs)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_appeals(path: &str) -> io::Result<Vec<Appeal>> {
    let content = fs::read_to_string(path)?;
    let appeals: Vec<Appeal> = serde_json::from_str(&content)?;
    Ok(appeals)
}

pub fn save_appeals(path: &str, appeals: &[Appeal]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(appeals)?;
    fs::write(path, json)?;
    Ok(())
}

// Admin/Appeal tools
pub fn print_logs(logs: &[ActionLog]) {
    for log in logs {
        println!("{:?}", log);
    }
}

pub fn check_blacklist(participants: &BTreeMap<String, Participant>, id: &str) {
    if let Some(p) = participants.get(id) {
        if p.blacklisted {
            println!("{} is blacklisted: {:?}", id, p.blacklist_reason);
        } else {
            println!("{} is not blacklisted", id);
        }
    } else {
        println!("Participant {} not found", id);
    }
}

pub fn submit_appeal_queue(appeals: &mut Vec<Appeal>, id: &str, reason: &str) {
    let now = Utc::now().timestamp();
    appeals.push(Appeal {
        participant_id: id.to_string(),
        reason: reason.to_string(),
        timestamp: now,
        reviewed: false,
        approved: false,
    });
    println!("Appeal queued for review: {}", id);
}

pub fn review_appeal(
    appeals: &mut Vec<Appeal>,
    participants: &mut BTreeMap<String, Participant>,
    idx: usize,
    approve: bool,
) {
    if let Some(app) = appeals.get_mut(idx) {
        app.reviewed = true;
        app.approved = approve;
        if approve {
            if let Some(p) = participants.get_mut(&app.participant_id) {
                p.blacklisted = false;
                p.strikes = 0;
                p.blacklist_reason = None;
                println!("Appeal approved and blacklist cleared for {}", app.participant_id);
            }
        } else {
            println!("Appeal rejected for {}", app.participant_id);
        }
    }
}

pub fn submit_appeal(participants: &mut BTreeMap<String, Participant>, id: &str, reason: &str) {
    if let Some(p) = participants.get_mut(id) {
        if p.blacklisted {
            println!("Appeal submitted for {}: {}", id, reason);
            // For demo: auto-clear blacklist
            p.blacklisted = false;
            p.strikes = 0;
            p.blacklist_reason = None;
        } else {
            println!("{} is not blacklisted", id);
        }
    } else {
        println!("Participant {} not found", id);
    }
}

pub fn reset_strikes(participants: &mut BTreeMap<String, Participant>, id: &str) {
    if let Some(p) = participants.get_mut(id) {
        p.strikes = 0;
        println!("Strikes reset for {}", id);
    } else {
        println!("Participant {} not found", id);
    }
}
