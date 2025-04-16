# Nexus zkVM Proof of Reserves POC 

A POC on Proof of Reserves system using the [Nexus zkVM SDK](https://github.com/nexus-xyz/nexus-zkvm). 

---

## What Does It Prove?

- Computes a zero-knowledge proof that reserves (BTC, ETH) meet or exceed thresholds (**Proof of Reserves**).
- Computes a zero-knowledge proof that total assets (BTC + ETH) are greater than or equal to total liabilities (**Proof of Solvency**).
- Verifies that reserves are operated by the bank itself (`reserve_operator == bank_name`).
- Allows issuing and checking of **Soulbound Tokens (SBTs)** for user attributes (e.g., KYC), which are persistent and non-transferable.
- Supports a combined command to check reserves, solvency, and SBT status in one go (**check-all**).
- All proofs and checks are performed without revealing underlying sensitive balances or user data.

---

## Project Structure

```
nexus-host/
├── Cargo.toml
├── sbts.json                # SBT registry (persistent)
├── README.md
├── src/
│   ├── main.rs              # Host: CLI entry point (reserves, solvency, SBTs, check-all)
│   ├── lib.rs               # Shared logic and struct definitions
│   └── guest/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs      # Guest: proof logic
│           └── input.rs     # Input struct for multi-asset proof
├── rust-toolchain.toml
└── ... (other files)
```

---

## Prerequisites
- Rust nightly toolchain (see `rust-toolchain.toml`)
- [Nexus zkVM SDK requirements](https://github.com/nexus-xyz/nexus-zkvm)

---

## Proof of Reserves, Proof of Solvency, and Soulbound Tokens (SBTs)

### Features
- **Proof of Reserves**: Prove that total BTC and ETH balances meet specified thresholds.
- **Proof of Solvency**: Prove that total assets (BTC + ETH) are greater than or equal to total liabilities.
- **Soulbound Tokens (SBTs)**: Issue and check non-transferable user attributes (e.g., KYC) that persist across runs.
- **Combined Check**: Run all three checks in a single command for convenience.

---

### Usage

#### 1. Proof of Reserves
```
cargo run -- <btc_balances> <eth_balances> <btc_threshold> <eth_threshold> <bank_name> <reserve_operator>
# Example:
cargo run -- 100,200 50,50 100 50 MyBank MyBank
```

#### 2. Proof of Solvency
```
cargo run -- prove-solvency <btc_balances> <eth_balances> <liabilities> <bank_name> <reserve_operator>
# Example:
cargo run -- prove-solvency 100,200 50,50 250,100 MyBank MyBank
```

#### 3. Soulbound Tokens (SBTs)
- **Issue an SBT**
  ```
  cargo run -- issue-sbt <user> <attribute> <issuer>
  # Example:
  cargo run -- issue-sbt alice KYC Exchange
  ```
- **Check an SBT**
  ```
  cargo run -- check-sbt <user> <attribute>
  # Example:
  cargo run -- check-sbt alice KYC
  ```

#### 4. Combined Check (Proof of Reserves, Solvency, and SBT)
```
cargo run -- check-all <btc_balances> <eth_balances> <btc_threshold> <eth_threshold> <liabilities> <bank_name> <reserve_operator> <sbt_user> <sbt_attribute>
# Example:
cargo run -- check-all 100,200 50,50 100 50 250,100 MyBank MyBank alice KYC
```

---

### Notes
- SBTs are persisted in `sbts.json` in the project directory.
- The CLI expects comma-separated lists for balances and liabilities (e.g., `100,200`).
- The code is designed for easy extension to more assets and SBT attributes.

---

### Code Structure
- `src/main.rs`: CLI entry point and logic for reserves, solvency, and SBTs
- `src/lib.rs`: Shared logic and struct definitions
- `src/guest/`: Guest program logic for zero-knowledge proofs

---

### Example Workflow
```sh
# Issue a KYC SBT for alice
cargo run -- issue-sbt alice KYC Exchange

# Check SBT for alice
cargo run -- check-sbt alice KYC

# Prove reserves
cargo run -- 100,200 50,50 100 50 MyBank MyBank

# Prove solvency
cargo run -- prove-solvency 100,200 50,50 250,100 MyBank MyBank

# Run all checks
cargo run -- check-all 100,200 50,50 100 50 250,100 MyBank MyBank alice KYC
```

---

### Requirements
- Rust
- [nexus_sdk](https://github.com/NehharShah/nexus-por) and dependencies

---

### Extending
- To add new assets, extend `MultiAssetProofInput` and update guest/main.rs logic.
- To add new SBT attributes or issuers, simply use new values in the CLI.

---

For further details, see the code comments and each command's usage output.

---

## License

MIT