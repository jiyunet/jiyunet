
use crypto::{sha2, ed25519};
use crypto::digest::Digest;

use DagComponent;
use DecodeError;

pub const SHA256_WIDTH: usize = 32;

#[derive(Copy)]
pub struct Hash([u8; SHA256_WIDTH]);

impl Hash {

    pub fn new(hex: [u8; SHA256_WIDTH]) -> Hash {
        Hash(hex)
    }

    pub fn from_blob(blob: &[u8]) -> Hash {

        let mut hasher = sha2::Sha256::new();
        hasher.input(blob);

        let mut hashed = [0; SHA256_WIDTH];
        hasher.result(&mut hashed);
        Hash::new(hashed)

    }

    pub fn into_fingerprint(self) -> Fingerprint {
        Fingerprint(self)
    }

}

impl Clone for Hash {
    fn clone(&self) -> Self {
        *self // REEE
    }
}

#[derive(Copy, Clone)]
pub struct Fingerprint(Hash);

impl Fingerprint {
    pub fn new(hex: [u8; SHA256_WIDTH]) -> Fingerprint {
        Fingerprint(Hash::new(hex))
    }

    pub fn into_hash(self) -> Hash { // TODO Make this all From<T> or whatever.
        let Fingerprint(hash) = self; // Pattern matching! ^_^
        hash
    }

    pub fn into_array(self) -> [u8; SHA256_WIDTH] {
        let Fingerprint(Hash(s)) = self; // Even more pattern matching! ^_^
        s
    }
}

#[derive(Copy)]
pub enum Signature {
    Ed25519([u8; 64], Fingerprint)
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        *self
    }
}

impl Signature {

    pub fn specifier(&self) -> u8 {
        use self::Signature::*;
        match self {
            &Ed25519(_, _) => 0
        }
    }

    pub fn into_fingerprint(self) -> Fingerprint {
        match self {
            Signature::Ed25519(_, f) => f,
        }
    }

}

impl DagComponent for Signature {

    fn from_blob(data: &[u8]) -> Result<Self, DecodeError> {
        use self::Signature::*;
        match data[0] {
            0 => {
                if data.len() > 64 + 32 + 1 { // FIXME This is such a bad way to do it.

                    let mut tok = [0; 64];
                    for i in 0..64 {
                        tok[i] = data[i];
                    }

                    let mut fp = [0; 32];
                    for i in 0..32 {
                        fp[i] = data[i + 64];
                    }

                    Ok(Ed25519(tok, Fingerprint::new(fp)))
                    
                } else {
                    Err(DecodeError)
                }

            }
            _ => Err(DecodeError)
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        use self::Signature::*;

        let mut buf = Vec::new();
        buf.push(self.specifier());

        match self {
            &Ed25519(t, f) => {
                buf.extend_from_slice(&t);
                buf.extend_from_slice(&f.into_array());
            }
        }

        buf

    }

}

/// Scheme(private, public)
pub enum Keypair {
    Ed25519([u8; 64], [u8; 32])
}

// TODO Write implementation to deal with signing and stuff for Keypairs.
