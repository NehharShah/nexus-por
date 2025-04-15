mod lib;
use lib::{MultiAssetProofInput, prove_reserves_multi_asset};
use std::env;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};

const PACKAGE: &str = "guest";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <btc_balances_comma_separated> <eth_balances_comma_separated> <btc_threshold> <eth_threshold>", args[0]);
        std::process::exit(1);
    }
    let btc_balances: Vec<u64> = args[1].split(',').filter_map(|s| s.parse().ok()).collect();
    let eth_balances: Vec<u64> = args[2].split(',').filter_map(|s| s.parse().ok()).collect();
    let threshold_btc: u64 = args[3].parse().expect("Invalid BTC threshold");
    let threshold_eth: u64 = args[4].parse().expect("Invalid ETH threshold");
    let input = MultiAssetProofInput {
        btc_balances,
        eth_balances,
        threshold_btc,
        threshold_eth,
    };

    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with test verification

    print!("Proving proof-of-reserves... ");
    let (view, proof) = prover
        .prove_with_input::<MultiAssetProofInput, ()>(&input, &())
        .expect("failed to prove");

    let exit_code = view.exit_code().expect("failed to retrieve exit code");
    let logs = view.logs().expect("failed to retrieve debug logs");
    println!("All guest logs:");
    for (i, line) in logs.iter().enumerate() {
        println!("  [{}] {}", i, line);
    }

    // Updated log parser: handle PROOF_RESULT on same line or next line
    let mut proof_result: Option<u8> = None;
    for (i, log) in logs.iter().enumerate() {
        if let Some(idx) = log.find("PROOF_RESULT:") {
            let after_colon = log[(idx + "PROOF_RESULT:".len())..].trim();
            if !after_colon.is_empty() {
                if let Ok(val) = after_colon.parse::<u8>() {
                    proof_result = Some(val);
                    break;
                }
            } else if let Some(next) = logs.get(i + 1) {
                if let Ok(val) = next.trim().parse::<u8>() {
                    proof_result = Some(val);
                    break;
                }
            }
        }
    }
    let proof_result = proof_result.expect("PROOF_RESULT not found in guest logs");
    println!("Guest proof result: {}", proof_result);
    println!(
        ">>>>> Logging\n{}<<<<<",
        logs.join("")
    );
    if exit_code != 0 {
        eprintln!("Guest exited with error code {} - proof execution failed.", exit_code);
        std::process::exit(exit_code as i32);
    }
    if proof_result == 1 {
        println!("Proof of reserves succeeded: reserves meet threshold");
    } else {
        println!("Proof of reserves FAILED: reserves do NOT meet threshold");
        println!("Details:");
        println!("  BTC balances: {:?}, threshold: {}", input.btc_balances, input.threshold_btc);
        println!("  ETH balances: {:?}, threshold: {}", input.eth_balances, input.threshold_eth);
        println!("  Total BTC: {}", input.btc_balances.iter().sum::<u64>());
        println!("  Total ETH: {}", input.eth_balances.iter().sum::<u64>());
        // Do not panic; exit with code 2 for failed proof
        std::process::exit(2);
    }

    print!("Verifying proof...");
    let expected_output = 1u8;
    let verify_result = proof.verify_expected::<(), u8>(
        &(),
        0,
        &expected_output,
        &elf,
        &[],
    );
    match verify_result {
        Ok(_) => println!("  Succeeded!"),
        Err(e) => {
            eprintln!("Proof verification FAILED: {}", e);
            std::process::exit(3);
        }
    }

    let result = prove_reserves_multi_asset(&input);
    println!("{{\"proof_result\": {}}}", result);
    if result == 1 {
        println!("Proof of reserves succeeded: reserves meet threshold");
    } else {
        println!("Proof of reserves failed: reserves do not meet threshold");
    }
}
