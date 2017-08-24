use ident;

pub type Address = [u8; 32];

#[derive(Copy, Clone)]
pub struct Signature {
    signed_hash: [u8; 32],
    signer: Address
}

#[derive(Clone)]
pub struct Block {
    sig: Signature,
    version: u32,
    timestamp: i64,
    parents: Vec<Address>,
    segments: Vec<Segment>
}

#[derive(Clone)]
pub enum SegmentContent {
    IdentDecl(ident::Identity),
    Artifact(ArtifactData),
    ArtifactPointer(Address)
}

#[derive(Clone)]
pub struct Segment {
    sig: Signature,
    timestamp: i64,
    content: SegmentContent
}

#[derive(Clone)]
pub struct ArtifactData {
    spec: u16,
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

/// Used for storing artifacts off-chain, but still maintaining signature.
#[derive(Clone)]
pub struct Container {
    sig: Signature,
    timestamp: i64,
    art: ArtifactData
}
