use std::io::Cursor;

use byteorder::*; // FIXME

use core::Address;
use core::io::{BinaryComponent, DecodeError, WrResult};
use core::sig;
use core::sig::Signed;

use DagNode;

/// Main type of node on the dag.  Primary unit of time and validation.
///
/// Blocks have a header including their parent information.  They also contain a set of segments
/// that represent the data actually stored in the blocks.  Segments are for the actual mid-layer
/// validation logic.  One type of segments are artifact segments, which carry actual payload data
/// in the content layer.  Artifact contents have no bearing on validation, only total size.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {

    /// Version identifier.  TODO Formalize this.
    version: u32,

    /// Millisecond UNIX time
    timestamp: i64,

    /// The addresses of the immediate parent blocks (`Signed<Block>`) to this block.
    parents: Vec<Address>,

    /// The segments contained within this block.
    segments: Vec<Signed<Segment>>

}

impl BinaryComponent for Block {

    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_writer<W: WriteBytesExt>(&self, write: W) -> WrResult {
        unimplemented!();
    }

}

/// Any kind of data that can be stored in a segment.
///
/// * IdentDecl - Used to declare identities on the network.
/// * Artifact - Actual on-chain artifact.
/// * ArtifactPointer - Pointer to an artifact container, off-chain.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SegmentContent {
    IdentDecl(sig::Hash),
    Artifact(ArtifactData),
    ArtifactPointer(Address)
}

impl SegmentContent {

    /// Returns the speciier for the SegmentContent used for serialization.
    ///
    /// TODO Make this make more sense.
    fn specifier(&self) -> u8 {
        use self::SegmentContent::*;
        match self {
            &IdentDecl(_) => 0,
            &Artifact(_) => 1,
            &ArtifactPointer(_) => 2
        }
    }
}

impl BinaryComponent for SegmentContent {

    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_writer<W: WriteBytesExt>(&self, write: W) -> WrResult {
        unimplemented!();
    }

}

/// A segment itself, with a timestamp.  See the documentation for Block for more information.  You
/// probably want to use a `Signed<Segment>` if you're just working with them.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Segment {
    timestamp: i64,
    content: SegmentContent
}

impl Segment {

    /// Returns the actual segment content.
    pub fn content(&self) -> SegmentContent {
        self.content.clone()
    }

}

impl BinaryComponent for Segment {

    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_writer<W: WriteBytesExt>(&self, write: W) -> WrResult {
        unimplemented!();
    }

}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArtifactData {
    spec: u16, // Artifact code.  Big-endian 0xXXXY, where X is the namespace and Y is the subtype.
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

impl BinaryComponent for ArtifactData {

    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_writer<W: WriteBytesExt>(&self, write: W) -> WrResult {
        unimplemented!();
    }

}

/// The off-chain container.  Usually you would want to use it as a `Signed<ArtifactContainer>`.
/// You can technically chain these infinitely as it's actually a segment container, so you could
/// just chain `ArtifactPointer`s indefinitely.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArtifactContainer {
    version: u32,
    timestamp: i64,
    content: SegmentContent
}

impl BinaryComponent for ArtifactContainer {

    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_writer<W: WriteBytesExt>(&self, write: W) -> WrResult {
        unimplemented!();
    }

}

impl DagNode for Signed<ArtifactContainer> {

    fn version(&self) -> u32 {
        self.clone().extract().version
    }

    fn timestamp(&self) -> i64 {
        self.clone().extract().timestamp
    }

}

#[cfg(test)]
mod test {

    use super::*;
    use super::SegmentContent::*;

    #[test]
    fn ck_block_between_blob() {

        let block = Block {
            version: 42,
            timestamp: 1337,
            parents: vec![],
            segments: vec![]
        };

        assert_eq!(block, Block::from_blob(block.to_blob().as_slice()).unwrap().0)

    }

    #[test]
    fn ck_segment_between_blob_1() {

        let seg = Segment {
            timestamp: 123456789,
            content: IdentDecl(sig::Hash::from_blob(&[0, 1, 2, 3]))
        };

        assert_eq!(seg, Segment::from_blob(seg.to_blob().as_slice()).unwrap().0);

    }

    #[test]
    fn ck_segment_between_blob_2() {

        let seg = Segment {
            timestamp: 19101004,
            content: Artifact(ArtifactData {
                spec: 42,
                body: vec![65, 66, 67, 68]
            })
        };

        assert_eq!(seg, Segment::from_blob(seg.to_blob().as_slice()).unwrap().0);

    }

    #[test]
    fn ck_segment_between_blob_3() {

        let seg = Segment {
            timestamp: 80,
            content: ArtifactPointer(Address::of(&[1, 3, 3, 7]))
        };

        assert_eq!(seg, Segment::from_blob(seg.to_blob().as_slice()).unwrap().0);

    }

}
