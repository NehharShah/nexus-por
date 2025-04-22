#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;
mod input;
use input::MultiAssetProofInput;
mod policy;
use policy::{Participant, PolicyConfig, ActionLog};
use alloc::collections::BTreeMap;
use alloc::string::ToString;

#[nexus_rt::main]
fn main(input: MultiAssetProofInput) -> u8 {
    // Simulated participant registry (would be persistent in real system)
    static mut PARTICIPANTS: Option<BTreeMap<String, Participant>> = None;
    static mut ACTION_LOGS: Option<Vec<ActionLog>> = None;
    let config = PolicyConfig::default();
    let participant_id = input.bank_name.clone();
    let now = 0u64; // Replace with real timestamp if available

    unsafe {
        if PARTICIPANTS.is_none() {
            PARTICIPANTS = Some(BTreeMap::new());
        }
        if ACTION_LOGS.is_none() {
            ACTION_LOGS = Some(Vec::new());
        }
        let participants = PARTICIPANTS.as_mut().unwrap();
        let logs = ACTION_LOGS.as_mut().unwrap();
        let p = participants.entry(participant_id.clone()).or_insert(Participant {
            id: participant_id.clone(),
            strikes: 0,
            reputation: 100,
            blacklisted: false,
            blacklist_reason: None,
            last_action: None,
        });
        if p.blacklisted {
            println!("BLACKLISTED: {} - {}", p.id, p.blacklist_reason.clone().unwrap_or_default());
            logs.push(ActionLog {
                participant_id: p.id.clone(),
                action: "attempted_action_while_blacklisted".to_string(),
                timestamp: now,
                details: None,
            });
            return 0;
        }
        println!("GUEST: Starting proof-of-reserves");
        println!("GUEST: Bank name: {}", input.bank_name);
        println!("GUEST: Reserve operator: {}", input.reserve_operator);
        if input.reserve_operator != input.bank_name {
            println!("GUEST: Reserve operator does NOT match bank. Reserves are NOT verified.");
            println!("PROOF_RESULT: 0");
            p.strikes += 1;
            p.reputation += config.reputation_penalty;
            logs.push(ActionLog {
                participant_id: p.id.clone(),
                action: "reserve_operator_mismatch".to_string(),
                timestamp: now,
                details: None,
            });
            if p.strikes >= config.max_strikes {
                p.blacklisted = true;
                p.blacklist_reason = Some("Too many invalid actions".to_string());
                logs.push(ActionLog {
                    participant_id: p.id.clone(),
                    action: "blacklisted".to_string(),
                    timestamp: now,
                    details: p.blacklist_reason.clone(),
                });
            }
            return 0;
        }
        println!("GUEST: BTC balances: {:?}", input.btc_balances);
        println!("GUEST: ETH balances: {:?}", input.eth_balances);
        println!("GUEST: BTC threshold: {}", input.threshold_btc);
        println!("GUEST: ETH threshold: {}", input.threshold_eth);
        let sum_btc: u64 = input.btc_balances.iter().sum();
        let sum_eth: u64 = input.eth_balances.iter().sum();
        println!("Total BTC reserves: {}", sum_btc);
        println!("Total ETH reserves: {}", sum_eth);
        let btc_ok = sum_btc >= input.threshold_btc;
        let eth_ok = sum_eth >= input.threshold_eth;

        if let Some(liabilities) = &input.liabilities {
            let total_liabilities: u64 = liabilities.iter().sum();
            let total_assets = sum_btc + sum_eth;
            println!("Total liabilities: {}", total_liabilities);
            println!("Total assets: {}", total_assets);
            if total_assets >= total_liabilities {
                println!("PROOF_SOLVENCY: 1");
                logs.push(ActionLog {
                    participant_id: p.id.clone(),
                    action: "solvent".to_string(),
                    timestamp: now,
                    details: None,
                });
                return 1;
            } else {
                println!("PROOF_SOLVENCY: 0");
                p.strikes += 1;
                p.reputation += config.reputation_penalty;
                logs.push(ActionLog {
                    participant_id: p.id.clone(),
                    action: "insolvent".to_string(),
                    timestamp: now,
                    details: None,
                });
                if p.strikes >= config.max_strikes {
                    p.blacklisted = true;
                    p.blacklist_reason = Some("Too many insolvency proofs".to_string());
                    logs.push(ActionLog {
                        participant_id: p.id.clone(),
                        action: "blacklisted".to_string(),
                        timestamp: now,
                        details: p.blacklist_reason.clone(),
                    });
                }
                return 0;
            }
        }
        let result = if btc_ok && eth_ok { 1 } else { 0 };
        if result == 1 {
            logs.push(ActionLog {
                participant_id: p.id.clone(),
                action: "solvent".to_string(),
                timestamp: now,
                details: None,
            });
        } else {
            p.strikes += 1;
            p.reputation += config.reputation_penalty;
            logs.push(ActionLog {
                participant_id: p.id.clone(),
                action: "threshold_fail".to_string(),
                timestamp: now,
                details: None,
            });
            if p.strikes >= config.max_strikes {
                p.blacklisted = true;
                p.blacklist_reason = Some("Too many threshold failures".to_string());
                logs.push(ActionLog {
                    participant_id: p.id.clone(),
                    action: "blacklisted".to_string(),
                    timestamp: now,
                    details: p.blacklist_reason.clone(),
                });
            }
        }
        println!("PROOF_RESULT: {}", result);
        result
    }
}
