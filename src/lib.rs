//! Library for Nexus Proof of Reserves (PoR)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofInput {
    pub balances: Vec<u64>,
    pub threshold: u64,
}

impl ProofInput {
    pub fn new(balances: Vec<u64>, threshold: u64) -> Self {
        Self { balances, threshold }
    }
}

/// Returns 1 if sum(balances) >= threshold, else 0
pub fn prove_reserves(input: &ProofInput) -> u8 {
    let sum: u64 = input.balances.iter().sum();
    if sum >= input.threshold { 1 } else { 0 }
}
