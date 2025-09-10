use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attestation {
    pub atf_digest: String,
    pub signer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ESGMetadata {
    pub standard: String,
    pub category: String,
    pub geo: String,
    pub carbon_value: f64,
    pub cycle: String,
    pub digest: String,
    pub physical_id: String,
    pub attestation: Attestation,
    pub status: String,
    pub evidence: String,
}

impl ESGMetadata {
    pub fn new(category: &str, geo: &str, carbon_value: f64, cycle: &str) -> Self {
        let raw = format!("{}:{}:{}:{}", category, geo, carbon_value, cycle);
        let mut hasher = Sha3_512::new();
        hasher.update(raw.as_bytes());
        let digest = format!("{:x}", hasher.finalize());

        ESGMetadata {
            standard: "ERC-ESG/1.0".to_string(),
            category: category.to_string(),
            geo: geo.to_string(),
            carbon_value,
            cycle: cycle.to_string(),
            digest,
            physical_id: "seal:UNDEFINED".to_string(),
            attestation: Attestation {
                atf_digest: digest.clone(),
                signer: "did:atf:ai:UNDEFINED".to_string(),
            },
            status: "issued".to_string(),
            evidence: "cid:UNDEFINED".to_string(),
        }
    }
}

/// Simula `mintESGToken`
pub fn mint_esg_token(meta: ESGMetadata) -> ESGMetadata {
    ESGMetadata { status: "issued".to_string(), ..meta }
}

/// Simula `auditESGToken`
pub fn audit_esg_token(mut meta: ESGMetadata, new_digest: &str) -> ESGMetadata {
    meta.attestation.atf_digest = new_digest.to_string();
    meta.status = "audited".to_string();
    meta
}

/// Simula `retireESGToken`
pub fn retire_esg_token(mut meta: ESGMetadata, reason: &str) -> ESGMetadata {
    meta.status = format!("retired:{}", reason);
    meta
}
