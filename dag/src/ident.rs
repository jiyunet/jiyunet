use crypto::{ed25519, sha2};

#[derive(Copy, Clone)]
pub struct Identity {
    key: [u8; 32] // Ed25519 public key.
}
