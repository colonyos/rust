//! ColonyOS Crypto implementation
//! Uses k256 for secp256k1 ECDSA and SHA3-256 for hashing

use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha3::{Digest, Sha3_256};

/// Generate a new random private key
/// Returns: Hex-encoded private key (64 characters)
pub fn gen_prvkey() -> String {
    let signing_key = SigningKey::random(&mut OsRng);
    hex::encode(signing_key.to_bytes())
}

/// Derive the public ID from a private key
/// Uses SHA3-256 hash of "04" + hex(publicKey)
/// Returns: Hex-encoded ID (64 characters)
pub fn gen_id(private_key: &str) -> String {
    let private_key_bytes = hex::decode(private_key).expect("Invalid hex private key");
    let signing_key = SigningKey::from_slice(&private_key_bytes).expect("Invalid private key");
    let verifying_key = signing_key.verifying_key();

    // Get uncompressed public key (65 bytes: 0x04 + 32 bytes x + 32 bytes y)
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key_point.as_bytes();

    // Convert to hex string (includes the 0x04 prefix)
    let public_key_hex = hex::encode(public_key_bytes);

    // Hash the hex string representation
    let mut hasher = Sha3_256::new();
    hasher.update(public_key_hex.as_bytes());
    let hash = hasher.finalize();

    hex::encode(hash)
}

/// Sign a message with a private key
/// Returns: Hex-encoded signature (130 characters: r + s + v)
pub fn gen_signature(message: &str, private_key: &str) -> String {
    let private_key_bytes = hex::decode(private_key).expect("Invalid hex private key");
    let signing_key = SigningKey::from_slice(&private_key_bytes).expect("Invalid private key");

    // Hash message with SHA3-256
    let mut hasher = Sha3_256::new();
    hasher.update(message.as_bytes());
    let msg_hash = hasher.finalize();

    // Sign using RFC 6979 deterministic k
    let (signature, recovery_id) = signing_key
        .sign_prehash_recoverable(&msg_hash)
        .expect("Signing failed");

    // Format: r (32 bytes) + s (32 bytes) + v (1 byte)
    let r = signature.r().to_bytes();
    let s = signature.s().to_bytes();
    let v = recovery_id.to_byte();

    let mut sig_bytes = [0u8; 65];
    sig_bytes[..32].copy_from_slice(&r);
    sig_bytes[32..64].copy_from_slice(&s);
    sig_bytes[64] = v;

    hex::encode(sig_bytes)
}

/// Hash a message with SHA3-256
/// Returns: Hex-encoded hash (64 characters)
pub fn gen_hash(message: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

/// Recover the public ID from a message and signature
/// Returns: Hex-encoded ID (64 characters)
pub fn recid(message: &str, signature: &str) -> String {
    use k256::ecdsa::RecoveryId;

    let sig_bytes = hex::decode(signature).expect("Invalid hex signature");
    if sig_bytes.len() != 65 {
        panic!("Invalid signature length");
    }

    // Parse signature
    let r = &sig_bytes[..32];
    let s = &sig_bytes[32..64];
    let v = sig_bytes[64];

    let mut sig_bytes_rs = [0u8; 64];
    sig_bytes_rs[..32].copy_from_slice(r);
    sig_bytes_rs[32..].copy_from_slice(s);

    let signature = Signature::from_slice(&sig_bytes_rs).expect("Invalid signature");
    let recovery_id = RecoveryId::from_byte(v).expect("Invalid recovery id");

    // Hash message
    let mut hasher = Sha3_256::new();
    hasher.update(message.as_bytes());
    let msg_hash = hasher.finalize();

    // Recover verifying key
    let verifying_key =
        VerifyingKey::recover_from_prehash(&msg_hash, &signature, recovery_id).expect("Recovery failed");

    // Get uncompressed public key
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key_point.as_bytes();
    let public_key_hex = hex::encode(public_key_bytes);

    // Hash to get ID
    let mut hasher = Sha3_256::new();
    hasher.update(public_key_hex.as_bytes());
    let hash = hasher.finalize();

    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_prvkey() {
        let p = gen_prvkey();
        assert_eq!(64, p.len());

        // Keys should be unique
        let p2 = gen_prvkey();
        assert_ne!(p, p2);
    }

    #[test]
    fn test_gen_id() {
        let p = gen_prvkey();
        let id = gen_id(&p);
        assert_eq!(64, id.len());
    }

    #[test]
    fn test_gen_id_deterministic() {
        // Same private key should produce same ID
        let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";
        let id1 = gen_id(prvkey);
        let id2 = gen_id(prvkey);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_gen_signature() {
        let p = gen_prvkey();
        let msg = "test";
        let s = gen_signature(msg, &p);
        assert_eq!(130, s.len());
    }

    #[test]
    fn test_gen_hash() {
        let msg = "test";
        let h = gen_hash(msg);
        assert_eq!(64, h.len());
    }

    #[test]
    fn test_gen_hash_deterministic() {
        let msg = "hello world";
        let h1 = gen_hash(msg);
        let h2 = gen_hash(msg);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_recid() {
        let p = gen_prvkey();
        let id = gen_id(&p);
        let msg = "hello";
        let s = gen_signature(msg, &p);
        let rid = recid(msg, &s);
        assert_eq!(id, rid);
    }

    #[test]
    fn test_known_key() {
        // Test with a known key from the Go/Python implementation
        let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";
        let expected_id = "3fc05cf3df4b494e95d6a3d297a34f19938f7daa7422ab0d4f794454133341ac";
        let id = gen_id(prvkey);
        assert_eq!(expected_id, id);
    }
}
