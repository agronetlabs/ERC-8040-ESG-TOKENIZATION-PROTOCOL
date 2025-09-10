# AgroCrypto Quantum Core

[![Crates.io](https://img.shields.io/crates/v/agrocrypto-quantum-core.svg)](https://crates.io/crates/agrocrypto-quantum-core)
[![Docs](https://docs.rs/agrocrypto-quantum-core/badge.svg)](https://docs.rs/agrocrypto-quantum-core)

Reference implementation for the **AgroCrypto Quantum Governance — ESG Tokenization Protocol (EIP Draft)**.

## Example

```rust
use agrocrypto_quantum_core::{ESGMetadata, mint_esg_token, audit_esg_token, retire_esg_token};

fn main() {
    let meta = ESGMetadata::new("carbon", "BR-RS", 12.5, "2025-Q3");
    println!("Initial digest: {}", meta.digest);

    let issued = mint_esg_token(meta.clone());
    let audited = audit_esg_token(issued.clone(), "sha3-512:NEW_DIGEST");
    let retired = retire_esg_token(audited, "expired cycle");

    println!("Final status: {}", retired.status);
}

