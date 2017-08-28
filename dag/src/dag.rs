use sig;

use Address;
use DagComponent;
use DagNode;
use DecodeError;

#[derive(Clone)]
pub struct Signed<T> where T: DagComponent {
    signature: sig::Signature,
    body: T
}

impl<T> Signed<T> where T: DagComponent {

    pub fn new(kp: sig::Keypair, body: T) -> Signed<T> {
        Signed {
            signature: kp.sign(body.get_hash()),
            body: body
        }
    }

    pub fn extract(self) -> T {
        self.body
    }

}

impl<T> DagComponent for Signed<T> where T: DagComponent {

    fn from_blob(data: &[u8]) -> Result<Self, DecodeError> {

        let sr = sig::Signature::from_blob(data);
        if sr.is_err() {
            return Err(DecodeError); // Error out immediately.
        }

        let sig = sr.ok().unwrap();
        match T::from_blob(&data[sig.to_blob().len()..]) { // FIXME Not the best way to do this.
            Ok(b) => Ok(Signed { signature: sig, body: b }),
            Err(_) => Err(DecodeError)
        }

    }

    fn to_blob(&self) -> Vec<u8> {

        let mut buf = Vec::new();
        buf.append(&mut self.signature.to_blob());
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

impl DagNode for Signed<Block> {

    fn version(&self) -> u32 {
        self.body.version
    }

    fn timestamp(&self) -> i64 {
        self.body.timestamp
    }

}

#[derive(Clone)]
pub enum SegmentContent {
    IdentDecl(sig::Hash),
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
    spec: u16, // Artifact code.  Big-endian 0xXXXY, where X is the namespace and Y is the subtype.
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

#[derive(Clone)]
pub struct ArtifactContainer {
    version: u32,
    timestamp: i64,
    content: SegmentContent
}

impl DagComponent for ArtifactContainer {

    fn from_blob(_: &[u8]) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_blob(&self) -> Vec<u8> {
        unimplemented!();
    }

}

impl DagNode for Signed<ArtifactContainer> {

    fn version(&self) -> u32 {
        self.body.version
    }

    fn timestamp(&self) -> i64 {
        self.body.timestamp
    }

}
