# Nexus zkVM Proof of Reserves Example

This project demonstrates a minimal, extensible "Proof of Reserves" (PoR) system using the [Nexus zkVM SDK](https://github.com/nexus-xyz/nexus-zkvm). It is designed for cryptocurrency custodians, exchanges, and financial institutions to transparently and efficiently prove that they have sufficient reserves to cover user assets.

---

## What is Proof of Reserves (PoR)?

**Proof of Reserves (PoR)** is a cryptographic mechanism for exchanges/custodians to prove they hold enough assets to cover user deposits, increasing trust and transparency.

### **Purpose**
- Build trust and transparency in the crypto ecosystem.
- Demonstrate that exchanges/custodians can fulfill withdrawal requests.

### **Mechanism**
- Typically involves an independent auditor generating a snapshot of the platform's balance sheet.
- Uses Merkle trees to efficiently organize and verify user balances.
- Leverages on-chain transparency and periodic reporting.

### **Benefits**
- **Increased Trust:** Users can verify their assets are backed by sufficient reserves.
- **Reduced Insolvency Risk:** Mitigates the risk of exchanges failing to fulfill withdrawals.
- **Transparency:** Users and regulators can see how assets are managed.

### **Limitations**
- **Snapshot in Time:** Only proves reserves at a specific moment.
- **Off-chain Activities:** May not account for all liabilities or off-chain assets.
- **Potential for Hidden Liabilities:** Exchanges could still hide insolvency while appearing transparent.

### **Implementation Patterns**
- Independent third-party audits.
- Merkle trees for scalable user balance verification.
- On-chain proofs and blockchain explorers.
- Periodic audits and public reporting.

---

## Project Structure

```
nexus-host/
├── Cargo.toml
├── rust-toolchain.toml
├── .gitignore
├── src/
│   ├── main.rs             # Host: compiles, proves, and verifies the guest
│   └── guest/
│       ├── Cargo.toml
│       ├── rust-toolchain.toml
│       └── src/
│           └── main.rs     # Guest: proof-of-reserves logic
```

---

## What Does It Prove?

The guest program computes a simple proof:
- **Hardcoded balance:** `balance = 100`
- **Hardcoded threshold:** `threshold = 90`
- **Proof:** Shows that `balance >= threshold` holds, without revealing any other information.

The host program:
- Compiles and runs the guest in the zkVM.
- Parses the result from guest logs.
- Verifies the proof using Nexus SDK.

---

## Extensibility

This solution is designed to be **extensible**:
- **Multiple Balances:** Supports any number of balances (`Vec<u64>`) as input.
- **Configurable Thresholds:** Threshold can be set per proof.
- **Multi-Asset Support:** Easily extendable to support multiple assets (see code comments for example struct).
- **Multi-Branch/Entity:** Can be extended to prove reserves across branches/entities.
- **Integration:** Library and CLI can be embedded in any backend or automated workflow.

---

## Prerequisites

- Rust nightly toolchain (see `rust-toolchain.toml`)
- [Nexus zkVM SDK requirements](https://github.com/nexus-xyz/nexus-zkvm)
- Git

---

## Setup

1. **Clone the repository:**
    ```sh
    git clone https://github.com/NehharShah/nexus-por.git
    cd nexus-por/nexus-host
    ```

2. **Install Rust nightly toolchain:**
    ```sh
    rustup toolchain install nightly-2025-01-02
    rustup override set nightly-2025-01-02
    ```

3. **Build and run the host:**
    ```sh
    cargo run -r
    ```

---

## Output Example

```
Proving proof-of-reserves... Guest exit code: 0
All guest logs:
  [0] GUEST: Starting proof-of-reserves
  [1] PROOF_RESULT: 1

Guest proof result: 1
>>>>> Logging
GUEST: Starting proof-of-reserves
PROOF_RESULT: 1
<<<<<
Proof of reserves succeeded: reserves meet threshold
Verifying proof...  Succeeded!
```

---

## Customizing the Proof

- To change the balance or threshold, edit `src/guest/src/main.rs`:
    ```rust
    let balance = 100u64;
    let threshold = 90u64;
    ```
- For more complex logic (e.g., multiple balances, public/private inputs), see the Nexus zkVM SDK documentation.

---

## Example: Multi-Asset Input Struct

```rust
#[derive(Deserialize)]
pub struct MultiAssetProofInput {
    pub btc_balances: Vec<u64>,
    pub eth_balances: Vec<u64>,
    pub threshold_btc: u64,
    pub threshold_eth: u64,
    // Add more assets as needed
}
```

---

## Troubleshooting

- **Guest logs missing or proof fails:**  
  Ensure the guest and host input types match exactly. Start with no inputs, then incrementally add complexity.
- **Git errors:**  
  If you see `.git/index.lock` errors, remove the lock file:  
  `rm .git/index.lock`
- **Dependency issues:**  
  Ensure you are using the correct nightly toolchain and dependencies as specified.

---

## License

MIT

---

## Credits

- [Nexus zkVM](https://github.com/nexus-xyz/nexus-zkvm)
- Example and setup by [NehharShah](https://github.com/NehharShah)
