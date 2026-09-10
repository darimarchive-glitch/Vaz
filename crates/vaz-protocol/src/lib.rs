use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VazAddress(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainMessage {
    pub id: Uuid,
    pub from: VazAddress,
    pub to: VazAddress,
    pub subject: String,
    pub body: String,
    pub created_unix_ms: u64,
}

impl PlainMessage {
    pub fn new(from: VazAddress, to: VazAddress, subject: impl Into<String>, body: impl Into<String>, created_unix_ms: u64) -> Self {
        Self { id: Uuid::new_v4(), from, to, subject: subject.into(), body: body.into(), created_unix_ms }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedObject {
    pub version: u16,
    pub object_id: Uuid,
    pub sender_signing_public: [u8; 32],
    pub sender_ephemeral_public: [u8; 32],
    pub nonce: [u8; 24],
    pub ciphertext: Vec<u8>,
    pub signature: Vec<u8>,
}