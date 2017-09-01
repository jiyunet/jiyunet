
use crypto::{sha2, ed25519};
use crypto::digest::Digest;

use DagComponent;
use DagNode;
use DecodeError;

pub const SHA256_WIDTH: usize = 32;

#[derive(Copy, Eq, Ord, PartialOrd, Hash)]
pub struct Hash([u8; SHA256_WIDTH]);

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {

        let &Hash(a) = self;
        let &Hash(b) = other;
        a == b

    }
}

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

    pub fn info_array(self) -> [u8; SHA256_WIDTH] {
        let Hash(h) = self;
        h
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Scheme {
    Ed25519
}

impl Scheme {

    pub fn generate(self, seed: &[u8]) -> Keypair {
        match self {
            Scheme::Ed25519 => {
                let (kpriv, kpub) = ed25519::keypair(seed);
                Keypair::Ed25519(kpriv, kpub)
            }
        }
    }

    fn to_specifier(&self) -> u8 {
        use self::Scheme::*;
        match self {
            &Ed25519 => 0x00
        }
    }

    fn from_specifier(s: u8) -> Option<Scheme> {
        use self::Scheme::*;
        match s {
            0x00 => Some(Ed25519),
            _ => None
        }
    }

}

#[derive(Copy)]
pub enum Keypair {
    Ed25519([u8; 64], [u8; 32])
}

impl Clone for Keypair {
    fn clone(&self) -> Self {
        *self
    }
}

impl Keypair {

    pub fn sign(&self, hash: Hash) -> Signature {
        match self {
            &Keypair::Ed25519(kpriv, kpub) => {
                let q = ed25519::signature(&hash.info_array(), &kpriv);
                Signature::Ed25519(q, Fingerprint::new(kpub))
            }
        }
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

    fn scheme(&self) -> Scheme {
        use self::Signature::*;
        match self {
            &Ed25519(_, _) => Scheme::Ed25519
        }
    }

    pub fn into_fingerprint(self) -> Fingerprint {
        match self {
            Signature::Ed25519(_, f) => f,
        }
    }

}

impl DagComponent for Signature {

    fn from_blob(data: &[u8]) -> Result<(Self, usize), DecodeError> {
        use self::Signature::*;
        let sig_data = &data[1..];
        match Scheme::from_specifier(data[0]) {
            Some(s) => match s {
                Scheme::Ed25519 => {

                    // FIXME Make this more functional.  It looks horrible as-is.
                    if sig_data.len() > (64 + 32) {

                        let mut hex = [0; 64];
                        for i in 0..64 {
                            hex[i] = sig_data[i]
                        }

                        let mut fp = [0; 32];
                        for i in 0..32 {
                            fp[i] = sig_data[i + 64];
                        }

                        Ok((Ed25519(hex, Fingerprint::new(fp)), 64 + 32 + 1))

                    } else {
                        Err(DecodeError)
                    }

                }
            },
            None => Err(DecodeError)
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        use self::Signature::*;

        let mut buf = Vec::new();
        buf.push(self.scheme().to_specifier());

        match self {
            &Ed25519(t, f) => {
                buf.extend_from_slice(&t);
                buf.extend_from_slice(&f.into_array());
            }
        }

        buf

    }

}
