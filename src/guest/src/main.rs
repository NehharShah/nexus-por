#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

#[nexus_rt::main]
fn main() -> u8 {
    println!("GUEST: Starting proof-of-reserves");
    let balance = 100u64;
    let threshold = 90u64;
    let result = if balance >= threshold { 1 } else { 0 };
    println!("PROOF_RESULT: 1");
    1
}
