extern crate alloc;
use alloc::vec::Vec;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProofInput {
    pub balances: Vec<u64>,
    pub threshold: u64,
}
