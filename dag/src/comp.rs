use byteorder::*; // FIXME

use core::Address;
use core::io::{BinaryComponent, DecodeError, WrResult};
use core::sig;
use core::sig::Signed;

use DagNode;

/*
TODO Fix the matching for multivec_read.

macro_rules! multivec_read {
    ($rdr:ident ; $( $dtype:ty => $dest:ident ),+) => {
        let mut lens = vec![
            $(
                $rdr.read_u16::<BigEndian>()
                    .map_err(|_| DecodeError)
                    .expect(format!("Error trying to parse qty of {}", stringify!($dtype))
                        .to_str()),
            )+
        ];

        let mut i = 0; // Horrible hacks.
        $(
            for _ in 0..lens[i] {
                $dest.push($dtype::from_reader($rdr)?);
            }
            i += 1; // Horrible hacks.
        )+
    };
}

macro_rules! multivec_write {
    ($wtr:ident ; $( $src:expr ),+) => {
        $(
            if $src.len() >= 65536 {
                return Err(DecodeError);
            }
            $wtr.write_u16::<BigEndian>($src.len() as u16).map_err(|_| DecodeError)?;
        )+
        $(
            for e in $src {
                e.to_writer($wtr)?;
            }
        )+
    };
}
*/

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

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let ver = read.read_u32::<BigEndian>().map_err(|_| DecodeError)?;
        let ts = read.read_i64::<BigEndian>().map_err(|_| DecodeError)?;
        let pc = read.read_u8().map_err(|_| DecodeError)?;
        let sc = read.read_u16::<BigEndian>().map_err(|_| DecodeError)?;

        let mut pars = Vec::with_capacity(pc as usize);
        let mut segs = Vec::with_capacity(sc as usize);

        for _ in 0..pc {
            pars.push(Address::from_reader(read)?);
        }

        for _ in 0..sc {
            segs.push(Signed::from_reader(read)?); // This is kinda misleading, but the type inferencing works out.
        }

        Ok(Block {
            version: ver,
            timestamp: ts,
            parents: pars,
            segments: segs
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        write.write_u32::<BigEndian>(self.version).map_err(|_| ())?;
        write.write_i64::<BigEndian>(self.timestamp).map_err(|_| ())?;
        write.write_u8(self.parents.len() as u8).map_err(|_| ())?;
        write.write_u16::<BigEndian>(self.segments.len() as u16).map_err(|_| ())?;

        for a in &self.parents {
            a.to_writer(write)?;
        }

        for s in &self.segments {
            s.to_writer(write)?;
        }

        Ok(())

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

impl BinaryComponent for SegmentContent {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        use self::SegmentContent::*;
        match read.read_u8()? {
            0x00 => Ok(IdentDecl(sig::Hash::from_reader(read)?)),
            0x01 => Ok(Artifact(ArtifactData::from_reader(read)?)),
            0x02 => Ok(ArtifactPointer(Address::from_reader(read)?)),
            _ => Err(DecodeError)
        }
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        use self::SegmentContent::*;

        // Write the specifier.
        write.write_u8(match *self {
            IdentDecl(_) => 0x00,
            Artifact(_) => 0x01,
            ArtifactPointer(_) => 0x02
        }).map_err(|_| ())?;

        // Write the actual content.
        match self {
            &IdentDecl(h) => h.to_writer(write)?,
            &Artifact(ref ad) => ad.to_writer(write)?,
            &ArtifactPointer(ap) => ap.to_writer(write)?
        }

        Ok(())

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

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let ts = read.read_i64::<BigEndian>().map_err(|_| DecodeError)?;
        let cont = SegmentContent::from_reader(read)?;

        Ok(Segment {
            timestamp: ts,
            content: cont
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_i64::<BigEndian>(self.timestamp).map_err(|_| ())?;
        self.content.to_writer(write)?;
        Ok(())
    }

}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArtifactData {
    spec: u16, // Artifact code.  Big-endian 0xXXXY, where X is the namespace and Y is the subtype.
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

impl BinaryComponent for ArtifactData {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let sp = read.read_u16::<BigEndian>()?;
        let mut b = vec![0; read.read_u64::<BigEndian>()? as usize];
        read.read(b.as_mut_slice())?;

        Ok(ArtifactData {
            spec: sp,
            body: b
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_u16::<BigEndian>(self.spec).map_err(|_| ())?;
        write.write_u64::<BigEndian>(self.body.len() as u64).map_err(|_| ())?;
        write.write_all(self.body.as_slice()).map_err(|_| ())?;
        Ok(())
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

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let ver = read.read_u32::<BigEndian>().map_err(|_| DecodeError)?;
        let ts = read.read_i64::<BigEndian>().map_err(|_| DecodeError)?;
        let cont = SegmentContent::from_reader(read)?;

        Ok(ArtifactContainer {
            version: ver,
            timestamp: ts,
            content: cont
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_u32::<BigEndian>(self.version).map_err(|_| ())?;
        write.write_i64::<BigEndian>(self.timestamp).map_err(|_| ())?;
        self.content.to_writer(write)?;
        Ok(())
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

    use std::io::Cursor;

    use super::*;
    use super::SegmentContent::*;

    fn encode_and_decode<T: BinaryComponent>(t: T) -> T {

        let mut c = Cursor::new(Vec::new());
        t.to_writer(&mut c).unwrap();
        let d = c.into_inner();
        println!("{:?}", d);
        T::from_reader(&mut Cursor::new(d)).unwrap()

    }

    #[test]
    fn ck_block_between_blob() {

        let block = Block {
            version: 42,
            timestamp: 1337,
            parents: vec![],
            segments: vec![]
        };

        assert_eq!(block, encode_and_decode(block.clone()))

    }

    #[test]
    fn ck_segment_between_blob_1() {

        let seg = Segment {
            timestamp: 123456789,
            content: IdentDecl(sig::Hash::of_slice(&[1, 2, 3, 4]))
        };

        assert_eq!(seg, encode_and_decode(seg.clone()));

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

        assert_eq!(seg, encode_and_decode(seg.clone()));

    }

    #[test]
    fn ck_segment_between_blob_3() {

        let seg = Segment {
            timestamp: 80,
            content: ArtifactPointer(Address::of_slice(&[1, 3, 3, 7]))
        };

        assert_eq!(seg, encode_and_decode(seg.clone()));

    }

}
