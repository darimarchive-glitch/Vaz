use anyhow::{anyhow, Result};
use chacha20poly1305::{aead::{Aead, KeyInit}, XChaCha20Poly1305, XNonce};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hkdf::Hkdf;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use vaz_protocol::{EncryptedObject, PlainMessage, PROTOCOL_VERSION};
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};

pub struct IdentityKeys { pub signing: SigningKey, pub exchange_secret: StaticSecret }

impl IdentityKeys {
    pub fn generate() -> Self { let mut rng = OsRng; Self { signing: SigningKey::generate(&mut rng), exchange_secret: StaticSecret::random_from_rng(&mut rng) } }
    pub fn signing_public(&self) -> [u8; 32] { self.signing.verifying_key().to_bytes() }
    pub fn exchange_public(&self) -> [u8; 32] { XPublicKey::from(&self.exchange_secret).to_bytes() }
}

fn derive_key(shared: &[u8; 32], object_id: &[u8; 16]) -> Result<[u8; 32]> {
    let hk = Hkdf::<Sha256>::new(Some(b"VAZ-VMP-v1"), shared);
    let mut out = [0u8; 32];
    hk.expand(object_id, &mut out).map_err(|_| anyhow!("hkdf expand failed"))?;
    Ok(out)
}

pub fn seal_message(sender: &IdentityKeys, recipient_exchange_public: [u8; 32], message: &PlainMessage) -> Result<EncryptedObject> {
    let mut rng = OsRng;
    let ephemeral = StaticSecret::random_from_rng(&mut rng);
    let ephemeral_public = XPublicKey::from(&ephemeral);
    let shared = ephemeral.diffie_hellman(&XPublicKey::from(recipient_exchange_public)).to_bytes();
    let key = derive_key(&shared, message.id.as_bytes())?;
    let cipher = XChaCha20Poly1305::new((&key).into());
    let mut nonce = [0u8; 24]; rng.fill_bytes(&mut nonce);
    let plaintext = serde_json::to_vec(message)?;
    let ciphertext = cipher.encrypt(XNonce::from_slice(&nonce), plaintext.as_ref()).map_err(|_| anyhow!("encryption failed"))?;
    let mut signed = Vec::new();
    signed.extend_from_slice(message.id.as_bytes()); signed.extend_from_slice(&ephemeral_public.to_bytes()); signed.extend_from_slice(&nonce); signed.extend_from_slice(&ciphertext);
    let signature = sender.signing.sign(&signed).to_bytes().to_vec();
    Ok(EncryptedObject { version: PROTOCOL_VERSION, object_id: message.id, sender_signing_public: sender.signing_public(), sender_ephemeral_public: ephemeral_public.to_bytes(), nonce, ciphertext, signature })
}

pub fn open_message(recipient: &IdentityKeys, object: &EncryptedObject) -> Result<PlainMessage> {
    if object.version != PROTOCOL_VERSION { return Err(anyhow!("unsupported protocol version")); }
    let verifying = VerifyingKey::from_bytes(&object.sender_signing_public)?;
    let signature = Signature::from_slice(&object.signature)?;
    let mut signed = Vec::new();
    signed.extend_from_slice(object.object_id.as_bytes()); signed.extend_from_slice(&object.sender_ephemeral_public); signed.extend_from_slice(&object.nonce); signed.extend_from_slice(&object.ciphertext);
    verifying.verify(&signed, &signature)?;
    let shared = recipient.exchange_secret.diffie_hellman(&XPublicKey::from(object.sender_ephemeral_public)).to_bytes();
    let key = derive_key(&shared, object.object_id.as_bytes())?;
    let cipher = XChaCha20Poly1305::new((&key).into());
    let plaintext = cipher.decrypt(XNonce::from_slice(&object.nonce), object.ciphertext.as_ref()).map_err(|_| anyhow!("decryption failed"))?;
    let message: PlainMessage = serde_json::from_slice(&plaintext)?;
    if message.id != object.object_id { return Err(anyhow!("object id mismatch")); }
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*; use vaz_protocol::VazAddress;
    #[test]
    fn roundtrip() { let a=IdentityKeys::generate(); let b=IdentityKeys::generate(); let m=PlainMessage::new(VazAddress("alice@vaz.local".into()), VazAddress("bob@vaz.local".into()), "oi", "mensagem", 1); let e=seal_message(&a,b.exchange_public(),&m).unwrap(); assert_eq!(open_message(&b,&e).unwrap().body,"mensagem"); }
}