
use std::fmt::{Debug, Error, Formatter};

use crypto::{sha2, ed25519};
use crypto::digest::Digest;

use DagComponent;
use DecodeError;

pub const SHA256_WIDTH: usize = 32;

/// A SHA-256 hash.
#[derive(Copy, Ord, PartialOrd, Hash, Debug)]
pub struct Hash([u8; SHA256_WIDTH]);

impl Eq for Hash {}
impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {

        let &Hash(a) = self;
        let &Hash(b) = other;
        a == b

    }
}

impl Hash {
    /// Creates a new hash with the given contents.
    pub fn new(hex: [u8; SHA256_WIDTH]) -> Hash {
        Hash(hex)
    }

    /// Computes the SHA-256 hash of the given blob.
    pub fn from_blob(blob: &[u8]) -> Hash {

        let mut hasher = sha2::Sha256::new();
        hasher.input(blob);

        let mut hashed = [0; SHA256_WIDTH];
        hasher.result(&mut hashed);
        Hash::new(hashed)

    }

    /// Converts this hash into a `Fingerprint`, presumably used for creating a `Signature`.
    pub fn into_fingerprint(self) -> Fingerprint {
        Fingerprint(self)
    }

    /// (does what it says on the tin)
    pub fn into_array(self) -> [u8; SHA256_WIDTH] {
        let Hash(h) = self;
        h
    }
}

impl Clone for Hash {
    fn clone(&self) -> Self {
        *self // REEE
    }
}

/// The SHA-256 hash of an identity declaration artifact.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Fingerprint(Hash);

impl Fingerprint {
    pub fn new(hex: [u8; SHA256_WIDTH]) -> Fingerprint {
        Fingerprint(Hash::new(hex))
    }

    pub fn into_hash(self) -> Hash {
        // TODO Make this all From<T> or whatever.
        let Fingerprint(hash) = self; // Pattern matching! ^_^
        hash
    }

    pub fn into_array(self) -> [u8; SHA256_WIDTH] {
        let Fingerprint(Hash(s)) = self; // Even more pattern matching! ^_^
        s
    }
}

/// A signature algorithm scheme.  Only supports Ed25519 for now.
/// FIXME There is a better way to keep track of these in a modular manner.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Scheme {
    Ed25519,
}

impl Scheme {
    /// Generates a new keypair using the scheme (ourselves) and the given seed,
    pub fn generate(self, seed: &[u8]) -> Keypair {
        match self {
            Scheme::Ed25519 => {
                let (kpriv, kpub) = ed25519::keypair(seed);
                Keypair::Ed25519(kpriv, kpub)
            }
        }
    }

    /// Returns the specifier byte for this signature scheme.
    fn to_specifier(&self) -> u8 {
        use self::Scheme::*;
        match self {
            &Ed25519 => 0x00,
        }
    }

    /// Returns the signature scheme from the given specifier byte.
    fn from_specifier(s: u8) -> Option<Scheme> {
        use self::Scheme::*;
        match s {
            0x00 => Some(Ed25519),
            _ => None,
        }
    }
}

/// A keypair using some signature algorithm.  Only support Ed25519 for now.
#[derive(Copy)]
pub enum Keypair {
    Ed25519([u8; 64], [u8; 32]),
}

impl Clone for Keypair {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for Keypair {}
impl PartialEq for Keypair {
    fn eq(&self, other: &Self) -> bool {
        use self::Keypair::*;
        match (*self, *other) {
            (Ed25519(ap, ak), Ed25519(bp, bk)) => arr_eq(&ap, &bp) && arr_eq(&ak, &bk),
            _ => false,
        }
    }
}

/// A public key used to validate `Signed<T>` structures.
#[derive(Copy)]
pub enum ValidationKey {
    Ed25519([u8; 64]),
}

impl Clone for ValidationKey {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for ValidationKey {}
impl PartialEq for ValidationKey {
    fn eq(&self, other: &Self) -> bool {
        use self::ValidationKey::*;
        match (*self, *other) {
            (Ed25519(a), Ed25519(b)) => arr_eq(&a, &b),
            _ => false,
        }
    }
}

impl Keypair {
    /// Generates a signature for the given hash using this keypair.
    pub fn sign(&self, hash: Hash) -> Signature {
        match self {
            &Keypair::Ed25519(kpriv, kpub) => {
                let q = ed25519::signature(&hash.into_array(), &kpriv);
                Signature::Ed25519(q, Fingerprint::new(kpub))
            }
        }
    }

    /// Converts this keypair into *just* the validation (public) key.
    pub fn into_pubkey(self) -> ValidationKey {
        match self {
            Keypair::Ed25519(k, _) => ValidationKey::Ed25519(k),
        }
    }
}

/// The actual signature data (signed hash) of some blob and the fingerprint of the keypair used to create it.  Supports multiple schemes.
#[derive(Copy)]
pub enum Signature {
    Ed25519([u8; 64], Fingerprint),
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for Signature {}
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        use self::Signature::*;
        match (*self, *other) {
            (Ed25519(a, af), Ed25519(b, bf)) => af == bf && arr_eq(&a, &b),
            _ => false,
        }
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("[sig]"); // This is okay because it's just binary data anyways.
        Ok(())
    }
}

impl Signature {
    /// Returns the signature scheme used for this signature.
    fn scheme(&self) -> Scheme {
        use self::Signature::*;
        match self {
            &Ed25519(_, _) => Scheme::Ed25519,
        }
    }

    /// Extracts the fingerprint of the public key of the signature.
    pub fn into_fingerprint(self) -> Fingerprint {
        match self {
            Signature::Ed25519(_, f) => f,
        }
    }
}

impl DagComponent for Signature {
    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {
        use self::Signature::*;
        let sig_data = &blob[1..];
        match Scheme::from_specifier(blob[0]) {
            Some(s) => {
                match s {
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
                }
            }
            None => Err(DecodeError),
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

pub enum SigVerificationError {
    IncompatibleSignatureSchemes,
    MismatchedKey,
}

pub fn verify(sig: Signature, vk: ValidationKey, data: &[u8]) -> Result<(), SigVerificationError> {
    use self::SigVerificationError::*;
    match (sig, vk) {
        (Signature::Ed25519(sd, _), ValidationKey::Ed25519(kd)) => {
            if ed25519::verify(data, &kd, &sd) {
                Ok(())
            } else {
                Err(MismatchedKey)
            }
        }
        _ => Err(IncompatibleSignatureSchemes),
    }
}

fn arr_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {

    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    return true;

}
