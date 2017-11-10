
use std::convert::{From, Into};
use std::fmt::{Debug, Error, Formatter};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crypto::{sha2, ed25519};
use crypto::digest::Digest;

use io::{BinaryComponent, WrResult};
use io::DecodeError;

pub const SHA256_WIDTH: usize = 32;

/// Generic type for a "signed" version of `T`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Signed<T> where T: BinaryComponent {
    signature: Signature,
    body: T
}

impl<T> Signed<T> where T: BinaryComponent {

    /// Creates a new signed verison of the given `T`, signed with the specified keypair.
    pub fn new(kp: Keypair, body: T) -> Signed<T> {
        Signed {
            signature: kp.sign(body.get_hash()),
            body: body
        }
    }

    /// Unwraps the contained type into its unsigned form.
    pub fn extract(self) -> T {
        self.body
    }

    /// Extracts a copy of the body.
    pub fn extract_owned(&self) -> T {
        self.body.clone()
    }

    /// Returns a copy of the internal signature data.
    pub fn sig(&self) -> Signature {
        self.signature.clone()
    }

}

impl<T> BinaryComponent for Signed<T> where T: BinaryComponent {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let sig = Signature::from_reader(read)?;
        let body = T::from_reader(read)?;

        Ok(Signed {
            signature: sig,
            body: body
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        self.signature.to_writer(write)?;
        self.body.to_writer(write)?;
        Ok(())
    }

}

/// A SHA-256 hash.
#[derive(Copy, Ord, PartialOrd, Hash, Debug)]
pub struct Hash([u8; SHA256_WIDTH]);

impl Hash {

    /// Creates a new hash with the given contents.
    pub fn new(hex: [u8; SHA256_WIDTH]) -> Hash {
        Hash(hex)
    }

    /// Returns the SHA-256 of the blob of data.
    pub fn of_slice(data: &[u8]) -> Hash {
        let mut hasher = sha2::Sha256::new();
        hasher.input(data);
        let mut out = [0u8; SHA256_WIDTH];
        hasher.result(&mut out);
        Hash::new(out)
    }

    /// Converts into the raw byte array form.
    pub fn into_array(self) -> [u8; SHA256_WIDTH] {
        self.0
    }

}

impl Clone for Hash {
    fn clone(&self) -> Self {
        *self // REEE
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        let &Hash(a) = self;
        let &Hash(b) = other;
        a == b
    }
}

impl Eq for Hash {}

impl Into<[u8; SHA256_WIDTH]> for Hash {
    fn into(self) -> [u8; SHA256_WIDTH] {
        self.into_array()
    }
}

impl BinaryComponent for Hash {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        let mut buf = [0; SHA256_WIDTH];
        read.read(&mut buf)?;
        Ok(Hash::new(buf))
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        let &Hash(d) = self;
        write.write(&d).map(|_| ()).map_err(|_| ())
    }

}

/// The SHA-256 hash of an identity declaration artifact.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Fingerprint(Hash);

impl Fingerprint {

    pub fn new(hash: Hash) -> Fingerprint {
        Fingerprint(hash)
    }

}

impl From<[u8; SHA256_WIDTH]> for Fingerprint {
    fn from(d: [u8; SHA256_WIDTH]) -> Fingerprint {
        Fingerprint::new(Hash::new(d))
    }
}

impl Into<[u8; SHA256_WIDTH]> for Fingerprint {
    fn into(self) -> [u8; SHA256_WIDTH] {
        self .0 .0
    }
}

impl Into<Hash> for Fingerprint {
    fn into(self) -> Hash {
        self.0
    }
}

impl BinaryComponent for Fingerprint {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        Ok(Fingerprint::new(Hash::from_reader(read)?))
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        self.0.to_writer(write)
    }

}

/// A signature algorithm scheme.  Only supports Ed25519 for now.
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

    pub fn from_specifier(s: u8) -> Option<Scheme> {
        use self::Scheme::*;
        match s {
            0x00 => Some(Ed25519),
            _ => None
        }
    }

    pub fn to_specifier(&self) -> u8 {
        use self::Scheme::*;
        match *self {
            Ed25519 => 0x00
        }
    }

}

/// A keypair using some signature algorithm.  Only support Ed25519 for now.
#[derive(Copy)]
pub enum Keypair {

    /// Edwards curve signature algorithm.  `(private key, public key)`
    Ed25519([u8; 64], [u8; 32])

}

impl Keypair {

    /// Generates a signature for the given hash using this keypair.
    pub fn sign(&self, hash: Hash) -> Signature {
        match self {
            &Keypair::Ed25519(kpriv, kpub) => {
                let q = ed25519::signature(&hash.into_array(), &kpriv);
                Signature::Ed25519(q, Fingerprint::new(Hash::of_slice(&kpub)))
            }
        }
    }

}

impl Into<ValidationKey> for Keypair {
    fn into(self) -> ValidationKey {
        match self {
            Keypair::Ed25519(_, k) => ValidationKey::Ed25519(k)
        }
    }
}

impl Clone for Keypair {
    fn clone(&self) -> Self {
        *self
    }
}

#[allow(unreachable_patterns)]
impl PartialEq for Keypair {
    fn eq(&self, other: &Self) -> bool {
        use self::Keypair::*;
        match (*self, *other) {
            (Ed25519(ap, ak), Ed25519(bp, bk)) => arr_eq(&ap, &bp) && arr_eq(&ak, &bk),
            _ => false,
        }
    }
}

impl Eq for Keypair {}

/// A public key used to validate `Signed<T>` structures.
#[derive(Copy, Hash, Debug)]
pub enum ValidationKey {
    Ed25519([u8; 32]),
}

impl Into<Fingerprint> for ValidationKey {
    fn into(self) -> Fingerprint {
        use self::ValidationKey::*;
        Fingerprint::new(match self {
            Ed25519(k) => Hash::of_slice(&k)
        })
    }
}

impl Clone for ValidationKey {
    fn clone(&self) -> Self {
        *self
    }
}

#[allow(unreachable_patterns)]
impl PartialEq for ValidationKey {
    fn eq(&self, other: &Self) -> bool {
        use self::ValidationKey::*;
        match (*self, *other) {
            (Ed25519(a), Ed25519(b)) => arr_eq(&a, &b),
            _ => false,
        }
    }
}

impl Eq for ValidationKey {}

impl BinaryComponent for ValidationKey {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        use self::ValidationKey::*;
        match read.read_u8().map_err(|_| DecodeError)? {
            0x00 => {
                let mut buf = [0; 32];
                read.read(&mut buf);
                Ok(Ed25519(buf))
            },
            _ => Err(DecodeError)
        }
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        use self::ValidationKey::*;

        write.write_u8(match self {
            &Ed25519(_) => 0x00
        }).map_err(|_| ())?;

        match self {
            &Ed25519(k) => write.write(&k).map(|_| ()).map_err(|_| ())?
        }

        Ok(())

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

#[allow(unreachable_patterns)]
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        use self::Signature::*;
        match (*self, *other) {
            (Ed25519(a, af), Ed25519(b, bf)) => af == bf && arr_eq(&a, &b),
            _ => false,
        }
    }
}

impl Eq for Signature {}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("[sig]")?; // This is okay because it's just binary data anyways.
        Ok(())
    }
}

impl Signature {

    /// Returns the signature scheme used for this signature.
    #[allow(dead_code)]
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

impl BinaryComponent for Signature {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        match Scheme::from_specifier(read.read_u8().map_err(|_| DecodeError)?) {
            Some(s) => match s {
                Scheme::Ed25519 => {
                    let mut sd = [0; 64];
                    read.read(&mut sd).map_err(|_| DecodeError)?;
                    let f = Fingerprint::from_reader(read)?;
                    Ok(Signature::Ed25519(sd, f))
                }
            },
            None => Err(DecodeError)
        }
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        match self {
            &Signature::Ed25519(t, f) => {
                write.write(&t).map_err(|_| ())?;
                f.to_writer(write)?;
            }
        }
        Ok(())
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SigVerificationError {
    FingerprintMismatch,
    SignatureSchemeMismatch,
    KeyMismatch,
}

#[allow(unreachable_patterns)] // Remove this when necessary.
/// Verifies that a `Signature`, `ValidationKey`, and binary data match properly.
pub fn verify(sig: Signature, vk: ValidationKey, data: &[u8]) -> Result<(), SigVerificationError> {
    use self::SigVerificationError::*;
    match (sig, vk) {
        (Signature::Ed25519(sd, _), ValidationKey::Ed25519(kd)) => {
            if ed25519::verify(data, &kd, &sd) {
                Ok(())
            } else {
                Err(KeyMismatch)
            }
        }
        _ => Err(SignatureSchemeMismatch),
    }
}

/// Verifies that a `Signed<T>` is correct, assuming the specified `ValidationKey`.
pub fn verify_signed<T: BinaryComponent>(st: &Signed<T>, vk: ValidationKey) -> Result<(), SigVerificationError> {

    use self::SigVerificationError::*;

    let sfp = st.sig().into_fingerprint(); // Signature FingerPrint

    if sfp != vk.into() {
        Err(FingerprintMismatch)
    } else {
        verify_signed(st, vk)
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
