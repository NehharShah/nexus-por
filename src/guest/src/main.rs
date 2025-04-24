#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;
mod input;
use input::MultiAssetProofInput;

#[nexus_rt::main]
fn main(input: MultiAssetProofInput) -> u8 {
    println!("GUEST: Starting proof-of-reserves");
    println!("GUEST: Bank name: {}", input.bank_name);
    println!("GUEST: Reserve operator: {}", input.reserve_operator);
    if input.reserve_operator != input.bank_name {
        println!("GUEST: Reserve operator does NOT match bank. Reserves are NOT verified.");
        println!("PROOF_RESULT: 0");
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
            return 1;
        } else {
            println!("PROOF_SOLVENCY: 0");
            return 0;
        }
    }
    let result = if btc_ok && eth_ok { 1 } else { 0 };
    println!("PROOF_RESULT: {}", result);
    result
}
