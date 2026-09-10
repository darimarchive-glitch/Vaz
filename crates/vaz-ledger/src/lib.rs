use anyhow::{anyhow, Result};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vaz_protocol::VazAddress;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityRecord { pub address: VazAddress, pub signing_public: [u8; 32], pub exchange_public: [u8; 32], pub sequence: u64 }

#[derive(Default)]
pub struct IdentityLedger { records: HashMap<String, IdentityRecord> }

impl IdentityLedger {
    pub fn get(&self, address: &str) -> Option<&IdentityRecord> { self.records.get(address) }
    pub fn register_genesis(&mut self, record: IdentityRecord) -> Result<()> {
        if record.sequence != 0 || self.records.contains_key(&record.address.0) { return Err(anyhow!("invalid genesis")); }
        self.records.insert(record.address.0.clone(), record); Ok(())
    }
    pub fn rotate(&mut self, next: IdentityRecord, signature: &[u8]) -> Result<()> {
        let current = self.records.get(&next.address.0).ok_or_else(|| anyhow!("unknown identity"))?;
        if next.sequence != current.sequence + 1 { return Err(anyhow!("bad sequence")); }
        let mut msg = Vec::new(); msg.extend_from_slice(next.address.0.as_bytes()); msg.extend_from_slice(&next.signing_public); msg.extend_from_slice(&next.exchange_public); msg.extend_from_slice(&next.sequence.to_le_bytes());
        VerifyingKey::from_bytes(&current.signing_public)?.verify(&msg, &Signature::from_slice(signature)?)?;
        self.records.insert(next.address.0.clone(), next); Ok(())
    }
}