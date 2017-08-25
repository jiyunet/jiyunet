use crypto::{ed25519, sha2};
use crypto::digest::Digest;

pub type Hash = [u8; 32];
pub type HashSig = Hash;

#[derive(Copy, Clone)]
pub struct Identity {
    key: [u8; 32] // Ed25519 public key.
}

impl Identity {
    pub fn to_blob(&self) -> Vec<u8> {
        let mut o = Vec::new();
        o.extend(self.key.iter());
        o
    }
}

pub struct Keypair {
    kpriv: [u8; 64],
    kpub: [u8; 32]
}

impl Keypair {

    pub fn sign(&self, data: Hash) -> HashSig {
        //let foo = ed25519::signature(&data, &self.kpriv);
        unimplemented!();
    }

    pub fn fingerprint(&self) -> Hash {

        let mut hasher = sha2::Sha256::new();
        hasher.input(&self.kpub);

        let mut hashed: [u8; 32] = Default::default();
        hasher.result(&mut hashed);
        hashed

    }

    pub fn into_ident(self) -> Identity {
        Identity { key: self.kpub }
    }

}
