use ident;

use crypto::sha2;
use crypto::digest::Digest;

pub type Address = ident::Hash;

#[derive(Copy, Clone)]
pub struct Signature {
    hash_sig: ident::HashSig,
    author: ident::Hash
}

impl Signature {
    pub fn to_blob(&self) -> Vec<u8> {

        let mut buf = Vec::new();
        buf.extend_from_slice(&self.hash_sig);
        buf.extend_from_slice(&self.author);
        buf

    }
}

pub struct DecodeError;

pub trait DagComponent where Self: Clone {

    fn from_blob(data: &[u8]) -> Result<Self, DecodeError>;
    fn to_blob(&self) -> Vec<u8>;

    fn get_hash(&self) -> ident::Hash {

        let mut hasher = sha2::Sha256::new();
        hasher.input(self.to_blob().as_slice());

        let mut hashed: [u8; 32] = Default::default();
        hasher.result(&mut hashed);
        hashed

    }

    fn to_signed(self, kp: ident::Keypair) -> Signed<Self> {
        Signed::new(kp, self)
    }

}

#[derive(Clone)]
pub struct Signed<T> where T: DagComponent {
    sig: Signature,
    body: T
}

impl<T> Signed<T> where T: DagComponent {

    pub fn new(kp: ident::Keypair, body: T) -> Signed<T> {
        Signed {
            sig: Signature {
                hash_sig: kp.sign(body.get_hash()),
                author: kp.fingerprint()
            },
            body: body
        }
    }

    pub fn extract(self) -> T {
        self.body
    }

}

impl<T> DagComponent for Signed<T> where T: DagComponent {

    fn from_blob(data: &[u8]) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut buf = Vec::new();
        buf.append(&mut self.sig.to_blob());
        buf.append(&mut self.body.to_blob());
        buf

    }

}

#[derive(Clone)]
pub struct Block {
    version: u32,
    timestamp: i64,
    parents: Vec<Address>,
    segments: Vec<Signed<Segment>>
}

impl DagComponent for Block {

    fn from_blob(_: &[u8]) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_blob(&self) -> Vec<u8> {
        unimplemented!();
    }

}

#[derive(Clone)]
pub enum SegmentContent {
    IdentDecl(ident::Identity),
    Artifact(ArtifactData),
    ArtifactPointer(Address)
}

#[derive(Clone)]
pub struct Segment {
    timestamp: i64,
    content: SegmentContent
}

impl DagComponent for Segment {

    fn from_blob(_: &[u8]) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_blob(&self) -> Vec<u8> {
        unimplemented!();
    }

}

#[derive(Clone)]
pub struct ArtifactData {
    spec: u16,
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

impl DagComponent for ArtifactData {

    fn from_blob(_: &[u8]) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_blob(&self) -> Vec<u8> {
        unimplemented!();
    }

}
