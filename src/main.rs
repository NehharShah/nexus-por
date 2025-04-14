use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};

const PACKAGE: &str = "guest";

fn main() {
    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with test verification

    // Example: single reserve, private
    let _balance = 100u64;
    // Public threshold
    let _threshold = 90u64;

    print!("Proving proof-of-reserves... ");
    let (view, proof) = prover
        .prove_with_input::<(), ()>(&(), &())
        .expect("failed to prove program");

    let exit_code = view.exit_code().expect("failed to retrieve exit code");
    let logs = view.logs().expect("failed to retrieve debug logs");
    println!("Guest exit code: {}", exit_code);
    println!("All guest logs:");
    for (i, line) in logs.iter().enumerate() {
        println!("  [{}] {}", i, line);
    }
    let proof_result = logs.iter().find_map(|line| {
        if let Some(rest) = line.strip_prefix("PROOF_RESULT: ") {
            rest.trim().parse::<u8>().ok()
        } else {
            None
        }
    }).expect("PROOF_RESULT not found in guest logs");
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
}
