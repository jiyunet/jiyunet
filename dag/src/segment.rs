use byteorder::*; // FIXME

use core::Address;
use core::io::{BinaryComponent, DecodeError, WrResult};
use core::sig;

use artifact::*;

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

#[cfg(test)]
mod test {

    use std::io::Cursor;

    use core::Address;
    use core::sig::Hash;

    use super::*;

    use artifact::ArtifactData;
    use segment::Segment;
    use segment::SegmentContent::*;

    fn encode_and_decode<T: BinaryComponent>(t: T) -> T {

        let mut c = Cursor::new(Vec::new());
        t.to_writer(&mut c).unwrap();
        let d = c.into_inner();
        println!("{:?}", d);
        T::from_reader(&mut Cursor::new(d)).unwrap()

    }

    #[test]
    fn ck_segment_between_blob_1() {

        let seg = Segment {
            timestamp: 123456789,
            content: IdentDecl(Hash::of_slice(&[1, 2, 3, 4]))
        };

        assert_eq!(seg, encode_and_decode(seg.clone()));

    }

    #[test]
    fn ck_segment_between_blob_2() {

        let seg = Segment {
            timestamp: 19101004,
            content: Artifact(ArtifactData::new(42, vec![65, 66, 67, 68]))
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
