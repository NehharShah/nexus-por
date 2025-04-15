#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;
mod input;
use input::ProofInput;

#[nexus_rt::main]
fn main(input: ProofInput) -> u8 {
    println!("GUEST: Starting proof-of-reserves");
    println!("GUEST: Balances input: {:?}", input.balances);
    println!("GUEST: Threshold input: {}", input.threshold);
    let sum: u64 = input.balances.iter().sum();
    println!("Total reserves: {}", sum);
    println!("Threshold: {}", input.threshold);
    let result = if sum >= input.threshold { 1 } else { 0 };
    println!("PROOF_RESULT: {}", result); // Use println! for single-line output
    result
}
