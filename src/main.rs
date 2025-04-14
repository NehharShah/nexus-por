use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use lib::{ProofInput, prove_reserves};
use std::env;
use bincode;

mod lib;

const PACKAGE: &str = "guest";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <balances_comma_separated> <threshold>", args[0]);
        std::process::exit(1);
    }
    let balances: Vec<u64> = args[1].split(',').filter_map(|s| s.parse().ok()).collect();
    let threshold: u64 = args[2].parse().expect("Invalid threshold");

    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with test verification

    let input = ProofInput::new(balances, threshold);
    let _input_bytes = bincode::serialize(&input).expect("failed to serialize input");

    print!("Proving proof-of-reserves... ");
    let (view, proof) = prover
        .prove_with_input::<ProofInput, ()>(&input, &())
        .expect("failed to prove");

    let exit_code = view.exit_code().expect("failed to retrieve exit code");
    let logs = view.logs().expect("failed to retrieve debug logs");
    println!("Guest exit code: {}", exit_code);
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
    assert_eq!(exit_code, 0, "Guest exited with error code {}", exit_code);
    assert_eq!(proof_result, 1, "Proof of reserves failed: reserves do not meet threshold");
    println!("Proof of reserves succeeded: reserves meet threshold");

    print!("Verifying proof...");
    let expected_output = 1u8;
    proof
        .verify_expected::<(), u8>(
            &(),
            0,
            &expected_output,
            &elf,
            &[],
        )
        .expect("failed to verify proof");

    println!("  Succeeded!");

    let result = prove_reserves(&input);
    println!("{{\"proof_result\": {}}}", result);
    if result == 1 {
        println!("Proof of reserves succeeded: reserves meet threshold");
    } else {
        println!("Proof of reserves failed: reserves do not meet threshold");
    }
}
