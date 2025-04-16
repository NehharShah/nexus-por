//! Library for Nexus Proof of Reserves (PoR)

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiAssetProofInput {
    pub btc_balances: Vec<u64>,
    pub eth_balances: Vec<u64>,
    pub threshold_btc: u64,
    pub threshold_eth: u64,
    pub bank_name: String,        
    pub reserve_operator: String,    
}

impl MultiAssetProofInput {
    pub fn new(btc_balances: Vec<u64>, eth_balances: Vec<u64>, threshold_btc: u64, threshold_eth: u64, bank_name: String, reserve_operator: String) -> Self {
        Self { btc_balances, eth_balances, threshold_btc, threshold_eth, bank_name, reserve_operator }
    }
}

pub fn prove_reserves_multi_asset(input: &MultiAssetProofInput) -> u8 {
    if input.reserve_operator != input.bank_name {
        return 0;
    }
    let sum_btc: u64 = input.btc_balances.iter().sum();
    let sum_eth: u64 = input.eth_balances.iter().sum();
    let btc_ok = sum_btc >= input.threshold_btc;
    let eth_ok = sum_eth >= input.threshold_eth;
    if btc_ok && eth_ok { 1 } else { 0 }
}