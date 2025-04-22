mod lib;
mod moderation;
use lib::{prove_reserves_multi_asset, MultiAssetProofInput};
use moderation::*;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::path::Path;

const PACKAGE: &str = "guest";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBT {
    pub user: String,
    pub attribute: String,
    pub issuer: String,
}

pub struct SBTRegistry {
    tokens: HashMap<String, Vec<SBT>>,
}

impl SBTRegistry {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(tokens) = serde_json::from_str::<HashMap<String, Vec<SBT>>>(&data) {
                return Self { tokens };
            }
        }
        Self::new()
    }
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        if let Ok(data) = serde_json::to_string_pretty(&self.tokens) {
            let _ = fs::write(path, data);
        }
    }
    pub fn issue(&mut self, user: String, attribute: String, issuer: String) {
        let sbt = SBT {
            user: user.clone(),
            attribute,
            issuer,
        };
        self.tokens.entry(user).or_default().push(sbt);
    }
    pub fn has(&self, user: &str, attribute: &str) -> bool {
        self.tokens
            .get(user)
            .map_or(false, |sbts| sbts.iter().any(|s| s.attribute == attribute))
    }
    pub fn print_for(&self, user: &str) {
        if let Some(sbts) = self.tokens.get(user) {
            println!("SBTs for {}: {:?}", user, sbts);
        } else {
            println!("No SBTs for {}", user);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sbt_file = "sbts.json";
    let mut sbt_registry = SBTRegistry::from_file(sbt_file);

    if args.len() >= 2 && args[1] == "issue-sbt" {
        if args.len() != 5 {
            eprintln!("Usage: {} issue-sbt <user> <attribute> <issuer>", args[0]);
            std::process::exit(1);
        }
        sbt_registry.issue(args[2].clone(), args[3].clone(), args[4].clone());
        sbt_registry.save_to_file(sbt_file);
        println!("SBT issued!");
        sbt_registry.print_for(&args[2]);
        return;
    }
    if args.len() >= 2 && args[1] == "check-sbt" {
        if args.len() != 4 {
            eprintln!("Usage: {} check-sbt <user> <attribute>", args[0]);
            std::process::exit(1);
        }
        let has = sbt_registry.has(&args[2], &args[3]);
        println!("User '{}' has SBT '{}': {}", args[2], args[3], has);
        sbt_registry.print_for(&args[2]);
        return;
    }

    if args.len() >= 2 && args[1] == "prove-solvency" {
        if args.len() != 7 {
            eprintln!("Usage: {} prove-solvency <btc_balances> <eth_balances> <liabilities> <bank_name> <reserve_operator>", args[0]);
            std::process::exit(1);
        }
        let btc_balances: Vec<u64> = args[2].split(',').filter_map(|s| s.parse().ok()).collect();
        let eth_balances: Vec<u64> = args[3].split(',').filter_map(|s| s.parse().ok()).collect();
        let liabilities: Vec<u64> = args[4].split(',').filter_map(|s| s.parse().ok()).collect();
        let bank_name = args[5].clone();
        let reserve_operator = args[6].clone();
        let input = MultiAssetProofInput {
            btc_balances,
            eth_balances,
            threshold_btc: 0,
            threshold_eth: 0,
            bank_name,
            reserve_operator,
            liabilities: Some(liabilities),
        };
        println!("Compiling guest program...");
        let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
        let prover: Stwo<Local> =
            Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");
        let elf = prover.elf.clone();
        print!("Proving proof-of-solvency... ");
        let (view, proof) = prover
            .prove_with_input::<MultiAssetProofInput, ()>(&input, &())
            .expect("failed to prove");
        let logs = view.logs().expect("failed to retrieve debug logs");
        println!("All guest logs:");
        for (i, line) in logs.iter().enumerate() {
            println!("  [{}] {}", i, line);
        }
        let mut proof_result: Option<u8> = None;
        for (i, log) in logs.iter().enumerate() {
            if let Some(idx) = log.find("PROOF_SOLVENCY:") {
                let after_colon = log[(idx + "PROOF_SOLVENCY:".len())..].trim();
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
        let proof_result = proof_result.expect("PROOF_SOLVENCY not found in guest logs");
        println!("Guest proof of solvency result: {}", proof_result);
        if proof_result == 1 {
            println!("Proof of solvency succeeded: assets >= liabilities");
        } else {
            println!("Proof of solvency FAILED: assets < liabilities");
        }
        return;
    }

    if args.len() >= 2 && args[1] == "check-all" {
        if args.len() != 11 {
            eprintln!("Usage: {} check-all <btc_balances> <eth_balances> <btc_threshold> <eth_threshold> <liabilities> <bank_name> <reserve_operator> <sbt_user> <sbt_attribute>", args[0]);
            std::process::exit(1);
        }
        let btc_balances: Vec<u64> = args[2].split(',').filter_map(|s| s.parse().ok()).collect();
        let eth_balances: Vec<u64> = args[3].split(',').filter_map(|s| s.parse().ok()).collect();
        let threshold_btc: u64 = args[4].parse().expect("Invalid BTC threshold");
        let threshold_eth: u64 = args[5].parse().expect("Invalid ETH threshold");
        let liabilities: Vec<u64> = args[6].split(',').filter_map(|s| s.parse().ok()).collect();
        let bank_name = args[7].clone();
        let reserve_operator = args[8].clone();
        let sbt_user = args[9].clone();
        let sbt_attribute = args[10].clone();
        // Proof of reserves
        println!("\n=== Proof of Reserves ===");
        let input = lib::MultiAssetProofInput {
            btc_balances: btc_balances.clone(),
            eth_balances: eth_balances.clone(),
            threshold_btc,
            threshold_eth,
            bank_name: bank_name.clone(),
            reserve_operator: reserve_operator.clone(),
            liabilities: None,
        };
        println!("Compiling guest program...");
        let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
        let prover1: Stwo<Local> =
            Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");
        let (view, _) = prover1
            .prove_with_input::<lib::MultiAssetProofInput, ()>(&input, &())
            .expect("failed to prove");
        let logs = view.logs().expect("failed to retrieve debug logs");
        for (i, line) in logs.iter().enumerate() {
            println!("  [{}] {}", i, line);
        }

        println!("\n=== Proof of Solvency ===");
        let input_sol = lib::MultiAssetProofInput {
            btc_balances,
            eth_balances,
            threshold_btc: 0,
            threshold_eth: 0,
            bank_name: bank_name.clone(),
            reserve_operator: reserve_operator.clone(),
            liabilities: Some(liabilities),
        };
        println!("Compiling guest program...");
        let mut prover_compiler2 = Compiler::<CargoPackager>::new(PACKAGE);
        let prover2: Stwo<Local> =
            Stwo::compile(&mut prover_compiler2).expect("failed to compile guest program");
        let (view, _) = prover2
            .prove_with_input::<lib::MultiAssetProofInput, ()>(&input_sol, &())
            .expect("failed to prove");
        let logs = view.logs().expect("failed to retrieve debug logs");
        for (i, line) in logs.iter().enumerate() {
            println!("  [{}] {}", i, line);
        }
        // SBT check
        println!("\n=== SBT Check ===");
        let has = sbt_registry.has(&sbt_user, &sbt_attribute);
        println!("User '{}' has SBT '{}': {}", sbt_user, sbt_attribute, has);
        sbt_registry.print_for(&sbt_user);
        return;
    }
    if args.len() >= 2 && args[1] == "print-logs" {
        let logs = load_logs("action_logs.json").unwrap();
        print_logs(&logs);
        return;
    }
    if args.len() >= 2 && args[1] == "check-blacklist" {
        let mut parts = load_participants("participants.json").unwrap();
        if args.len() < 3 { println!("Usage: check-blacklist <id>"); return; }
        check_blacklist(&parts, &args[2]);
        return;
    }
    if args.len() >= 2 && args[1] == "submit-appeal" {
        let mut appeals = load_appeals("appeals.json").unwrap();
        if args.len() < 4 { println!("Usage: submit-appeal <id> <reason>"); return; }
        submit_appeal_queue(&mut appeals, &args[2], &args[3]);
        save_appeals("appeals.json", &appeals).unwrap();
        return;
    }
    if args.len() >= 2 && args[1] == "reset-strikes" {
        let mut parts = load_participants("participants.json").unwrap();
        if args.len() < 3 { println!("Usage: reset-strikes <id>"); return; }
        reset_strikes(&mut parts, &args[2]);
        save_participants("participants.json", &parts).unwrap();
        return;
    }
    if args.len() >= 2 && args[1] == "list-appeals" {
        let appeals = load_appeals("appeals.json").unwrap();
        for (i, app) in appeals.iter().enumerate() {
            println!("Appeal #{}: {:?}", i, app);
        }
        return;
    }
    if args.len() >= 2 && args[1] == "review-appeal" {
        let mut appeals = load_appeals("appeals.json").unwrap();
        let mut parts = load_participants("participants.json").unwrap();
        if args.len() < 4 { println!("Usage: review-appeal <idx> <approve:1|0>"); return; }
        let idx: usize = args[2].parse().unwrap();
        let approve: bool = args[3] == "1";
        review_appeal(&mut appeals, &mut parts, idx, approve);
        save_appeals("appeals.json", &appeals).unwrap();
        save_participants("participants.json", &parts).unwrap();
        return;
    }
    if args.len() >= 2 && args[1] == "reload-policy" {
        let policy = load_policy("policy.toml").unwrap();
        println!("Policy loaded: {:?}", policy);
        return;
    }
    if args.len() >= 2 && args[1] == "prove-reserves" {
        if args.len() != 7 {
            eprintln!("Usage: {} prove-reserves <btc_balances> <eth_balances> <btc_threshold> <eth_threshold> <bank_name> <reserve_operator>", args[0]);
            std::process::exit(1);
        }
        let btc_balances: Vec<u64> = args[2].split(',').filter_map(|s| s.parse().ok()).collect();
        let eth_balances: Vec<u64> = args[3].split(',').filter_map(|s| s.parse().ok()).collect();
        let threshold_btc: u64 = args[4].parse().expect("Invalid BTC threshold");
        let threshold_eth: u64 = args[5].parse().expect("Invalid ETH threshold");
        let bank_name = args[6].clone();
        let reserve_operator = args[7].clone();
        let input = MultiAssetProofInput {
            btc_balances,
            eth_balances,
            threshold_btc,
            threshold_eth,
            bank_name,
            reserve_operator,
            liabilities: None,
        };
        println!("Compiling guest program...");
        let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
        let prover: Stwo<Local> =
            Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");
        let elf = prover.elf.clone();
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
        println!(">>>>> Logging\n{}<<<<<", logs.join(""));
        if exit_code != 0 {
            eprintln!(
                "Guest exited with error code {} - proof execution failed.",
                exit_code
            );
            std::process::exit(exit_code as i32);
        }
        if proof_result == 1 {
            println!("Proof of reserves succeeded: reserves meet threshold");
        } else {
            println!("Proof of reserves FAILED: reserves do NOT meet threshold");
            println!("Details:");
            println!(
                "  BTC balances: {:?}, threshold: {}",
                input.btc_balances, input.threshold_btc
            );
            println!(
                "  ETH balances: {:?}, threshold: {}",
                input.eth_balances, input.threshold_eth
            );
            println!("  Total BTC: {}", input.btc_balances.iter().sum::<u64>());
            println!("  Total ETH: {}", input.eth_balances.iter().sum::<u64>());
            std::process::exit(2);
        }
        print!("Verifying proof...");
        let expected_output = 1u8;
        let verify_result = proof.verify_expected::<(), u8>(&(), 0, &expected_output, &elf, &[]);
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
        return;
    }
    if args.len() != 7 {
        eprintln!("Usage: {} <btc_balances_comma_separated> <eth_balances_comma_separated> <btc_threshold> <eth_threshold> <bank_name> <reserve_operator>", args[0]);
        std::process::exit(1);
    }
    let btc_balances: Vec<u64> = args[1].split(',').filter_map(|s| s.parse().ok()).collect();
    let eth_balances: Vec<u64> = args[2].split(',').filter_map(|s| s.parse().ok()).collect();
    let threshold_btc: u64 = args[3].parse().expect("Invalid BTC threshold");
    let threshold_eth: u64 = args[4].parse().expect("Invalid ETH threshold");
    let bank_name = args[5].clone();
    let reserve_operator = args[6].clone();
    let input = MultiAssetProofInput::new(
        btc_balances,
        eth_balances,
        threshold_btc,
        threshold_eth,
        bank_name,
        reserve_operator,
    );

    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone();

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
    println!(">>>>> Logging\n{}<<<<<", logs.join(""));
    if exit_code != 0 {
        eprintln!(
            "Guest exited with error code {} - proof execution failed.",
            exit_code
        );
        std::process::exit(exit_code as i32);
    }
    if proof_result == 1 {
        println!("Proof of reserves succeeded: reserves meet threshold");
    } else {
        println!("Proof of reserves FAILED: reserves do NOT meet threshold");
        println!("Details:");
        println!(
            "  BTC balances: {:?}, threshold: {}",
            input.btc_balances, input.threshold_btc
        );
        println!(
            "  ETH balances: {:?}, threshold: {}",
            input.eth_balances, input.threshold_eth
        );
        println!("  Total BTC: {}", input.btc_balances.iter().sum::<u64>());
        println!("  Total ETH: {}", input.eth_balances.iter().sum::<u64>());
        std::process::exit(2);
    }

    print!("Verifying proof...");
    let expected_output = 1u8;
    let verify_result = proof.verify_expected::<(), u8>(&(), 0, &expected_output, &elf, &[]);
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
