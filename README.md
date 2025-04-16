# Nexus zkVM Proof of Reserves POC 

A POC on Proof of Reserves system using the [Nexus zkVM SDK](https://github.com/nexus-xyz/nexus-zkvm). 

---

## What Does It Prove?

- Computes a zero-knowledge proof that reserves (BTC, ETH) meet or exceed thresholds.
- Verifies that reserves are operated by the bank itself (`reserve_operator == bank_name`).
- No proof if reserves are not operated by the bank.

---

## Project Structure

```
nexus-host/
├── Cargo.toml
├── src/
│   ├── main.rs             # Host: compiles, proves, and verifies the guest
│   └── guest/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs     # Guest: proof logic
│           └── input.rs    # Input struct
```

---

## Prerequisites
- Rust nightly toolchain (see `rust-toolchain.toml`)
- [Nexus zkVM SDK requirements](https://github.com/nexus-xyz/nexus-zkvm)

---

## Usage

Build and run the host:

```sh
cargo run -- <btc_balances> <eth_balances> <btc_threshold> <eth_threshold> <bank_name> <reserve_operator>
```

Example:
```sh
cargo run -- 100,200 50,50 100 50 "MyBank" "MyBank"
```

- If `reserve_operator` does not match `bank_name`, the proof will fail and output `PROOF_RESULT: 0`.

---

## Example Output

```
GUEST: Starting proof-of-reserves
GUEST: Bank name: MyBank
GUEST: Reserve operator: MyBank
GUEST: BTC balances: [100, 200]
GUEST: ETH balances: [50, 50]
GUEST: BTC threshold: 100
GUEST: ETH threshold: 50
Total BTC reserves: 300
Total ETH reserves: 100
PROOF_RESULT: 1
Proof of reserves succeeded: reserves meet threshold
```

---

## Input Struct

```rust
pub struct MultiAssetProofInput {
    pub btc_balances: Vec<u64>,
    pub eth_balances: Vec<u64>,
    pub threshold_btc: u64,
    pub threshold_eth: u64,
    pub bank_name: String,
    pub reserve_operator: String,
}
```

---

## Customization
- Add more assets or thresholds by extending the struct and logic in `src/lib.rs` and `src/guest/src/input.rs`.
- All code is extensible for multi-asset, multi-branch, or other requirements.

---

## License

MIT