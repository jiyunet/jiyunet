use std::io::Cursor;

use byteorder::*; // FIXME

use sig;

use Address;
use DagComponent;
use DagNode;
use DecodeError;

/// Generic type for a "signed" version of `T`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Signed<T> where T: DagComponent {
    signature: sig::Signature,
    body: T
}

impl<T> Signed<T> where T: DagComponent {

    /// Creates a new signed verison of the given `T`, signed with the specified keypair.
    pub fn new(kp: sig::Keypair, body: T) -> Signed<T> {
        Signed {
            signature: kp.sign(body.get_hash()),
            body: body
        }
    }

    /// Unwraps the contained type into its unsigned form.
    pub fn extract(self) -> T {
        self.body
    }

}

impl<T> DagComponent for Signed<T> where T: DagComponent {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {
        match sig::Signature::from_blob(blob) {
            Ok((sig, len)) => match T::from_blob(&blob[len..]) {
                Ok((b, bl)) => Ok((Signed { signature: sig, body: b }, len + bl)),
                _ => Err(DecodeError)
            },
            _ => Err(DecodeError)
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut buf = Vec::new();
        buf.append(&mut self.signature.to_blob());
        buf.append(&mut self.body.to_blob());
        buf

    }

}

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

impl DagComponent for Block {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);

        let ver = c.read_u32::<BigEndian>()?;
        let ts = c.read_i64::<BigEndian>()?;
        let parent_cnt = c.read_u8()?;
        let seg_cnt = c.read_u16::<BigEndian>()?;

        let mut parents = Vec::new();
        let mut segments = Vec::new();
        let cpos = c.position() as usize;
        let mut inner = &c.into_inner()[cpos..];

        let mut consumed = 0;

        for _ in 0..parent_cnt {
            let (addr, len) = Address::from_blob(inner)?;
            parents.push(addr);
            inner = &inner[len..];
            consumed += len;
        }

        for _ in 0..seg_cnt {
            let (seg, len) = Signed::from_blob(inner)?;
            segments.push(seg);
            inner = &inner[len..];
            consumed += len;
        }

        Ok((Block {
            version: ver,
            timestamp: ts,
            parents: parents,
            segments: segments
        }, consumed + cpos))
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_u32::<BigEndian>(self.version).unwrap();
        v.write_i64::<BigEndian>(self.timestamp).unwrap();
        v.write_u8(self.parents.len() as u8).unwrap();
        v.write_u16::<BigEndian>(self.segments.len() as u16).unwrap();

        for p in &self.parents {
            v.append(&mut p.to_blob());
        }

        for seg in &self.segments {
            v.append(&mut seg.to_blob());
        }

        v

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

impl DagComponent for SegmentContent {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        use self::SegmentContent::*;

        let mut c = Cursor::new(blob);
        let read = c.position() as usize;
        let tag = c.read_u8()?;
        let dblob = &c.into_inner()[read..];

        match tag {
            0 => {
                if dblob.len() < sig::SHA256_WIDTH {
                    return Err(DecodeError);
                }

                let mut hd = [0; sig::SHA256_WIDTH];
                for i in 0..sig::SHA256_WIDTH {
                    hd[i] = dblob[i];
                }
                Ok((IdentDecl(sig::Hash::new(hd)), read + sig::SHA256_WIDTH))
            }

            1 => {
                let (ad, len) = ArtifactData::from_blob(dblob)?;
                Ok((Artifact(ad), read + len))
            }

            2 => {
                let (addr, len) = Address::from_blob(dblob)?;
                Ok((ArtifactPointer(addr), read + len))
            }

            _ => Err(DecodeError),
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        use self::SegmentContent::*;

        let mut v = Vec::new();
        v.push(self.specifier());
        match self {
            &IdentDecl(hash) => v.extend_from_slice(&hash.into_array()),
            &Artifact(ref a) => v.append(&mut a.to_blob()),
            &ArtifactPointer(p) => v.append(&mut p.to_blob())
        }

        v

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

impl DagComponent for Segment {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);
        let ts = c.read_i64::<BigEndian>()?;

        let read = c.position() as usize;
        let (d, sclen) = SegmentContent::from_blob(&c.into_inner()[read..])?;
        Ok((Segment {
            timestamp: ts,
            content: d
        }, read + sclen))
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_i64::<BigEndian>(self.timestamp).unwrap();
        v.append(&mut self.content.to_blob());
        v

    }

}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArtifactData {
    spec: u16, // Artifact code.  Big-endian 0xXXXY, where X is the namespace and Y is the subtype.
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

impl DagComponent for ArtifactData {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);
        let spec = c.read_u16::<BigEndian>()?;
        let len = c.read_u32::<BigEndian>()?;

        let read = c.position() as usize;
        let buf = &c.into_inner()[read .. read + len as usize];
        let mut v = Vec::new();
        v.extend_from_slice(buf);
        Ok((ArtifactData { spec: spec, body: v }, read + len as usize))
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_u16::<BigEndian>(self.spec).unwrap();
        v.write_u32::<BigEndian>(self.body.len() as u32).unwrap();
        v.extend_from_slice(self.body.as_slice());
        v

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

impl DagComponent for ArtifactContainer {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);
        let ver = c.read_u32::<BigEndian>()?;
        let ts = c.read_i64::<BigEndian>()?;

        let read = c.position() as usize;
        let (sc, slen) = SegmentContent::from_blob(&c.into_inner()[read..])?;

        Ok((ArtifactContainer {
            version: ver,
            timestamp: ts,
            content: sc
        }, read + slen))
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_u32::<BigEndian>(self.version).unwrap();
        v.write_i64::<BigEndian>(self.timestamp).unwrap();
        v.append(&mut self.content.to_blob());
        v

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
