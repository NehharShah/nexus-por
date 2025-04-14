# Nexus zkVM Proof of Reserves Example

This project demonstrates a minimal "Proof of Reserves" (PoR) using the [Nexus zkVM SDK](https://github.com/nexus-xyz/nexus-zkvm).  
It consists of a **host** program (verifier/prover) and a **guest** program (the logic to be proved), both written in Rust.

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
