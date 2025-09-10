# agrocrypto-quantum-core
Reference implementation for the AgroCrypto Quantum Governance — ESG Tokenization Protocol (EIP Draft).

# AgroCrypto Quantum Core

Reference implementation for the AgroCrypto Quantum Governance — ESG Tokenization Protocol (EIP Draft).

## Example

```rust
use agrocrypto_quantum_core::ESGMetadata;

fn main() {
    let meta = ESGMetadata::new("carbon", "BR-RS", 12.5, "2025-Q3");
    println!("Digest: {}", meta.digest);
}

