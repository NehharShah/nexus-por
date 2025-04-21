extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MultiAssetProofInput {
    pub btc_balances: Vec<u64>,
    pub eth_balances: Vec<u64>,
    pub threshold_btc: u64,
    pub threshold_eth: u64,
    pub bank_name: String,
    pub reserve_operator: String,
    pub liabilities: Option<Vec<u64>>,
    // Add more assets as needed
}
