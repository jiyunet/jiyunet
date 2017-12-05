use byteorder::*; // FIXME

use core::Address;
use core::io::{BinaryComponent, DecodeError, WrResult};
use core::sig::{Hash, Signed};

use segment::*;

use DagNode;

/// Main type of node on the dag.  Primary unit of time and validation.
///
/// Blocks have a header including their parent information.  They also contain a set of segments
/// that represent the data actually stored in the blocks.  Segments are for the actual mid-layer
/// validation logic.  One type of segments are artifact segments, which carry actual payload data
/// in the content layer.  Artifact contents have no bearing on validation, only total size.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlockHeader {

    /// Version identifier.  TODO Formalize this.
    version: u32,

    /// Millisecond UNIX time
    timestamp: i64,

    /// The merkle root of the segments in the block.
    segments_merkle_root: Hash,

    /// The addresses of the immediate parent blocks (`Signed<Block>`) to this block.
    parents: Vec<Address>

}

impl BlockHeader {

    pub fn parents(&self) -> Vec<Address> {
        self.parents.clone()
    }

}

impl BinaryComponent for BlockHeader {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let ver = read.read_u32::<BigEndian>().map_err(|_| DecodeError)?;
        let ts = read.read_i64::<BigEndian>().map_err(|_| DecodeError)?;
        let smr = Hash::from_reader(read)?;
        let pars = Vec::<Address>::from_reader(read)?;

        Ok(BlockHeader {
            version: ver,
            timestamp: ts,
            segments_merkle_root: smr,
            parents: pars
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        write.write_u32::<BigEndian>(self.version).map_err(|_| ())?;
        write.write_i64::<BigEndian>(self.timestamp).map_err(|_| ())?;
        self.segments_merkle_root.to_writer(write)?;
        self.parents.to_writer(write)?;

        Ok(())

    }

}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {
    header: Signed<BlockHeader>,
    body: Vec<Signed<Segment>>
}

impl BinaryComponent for Block {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let head = Signed::<BlockHeader>::from_reader(read)?;
        let body = Vec::<Signed<Segment>>::from_reader(read)?;

        Ok(Block {
            header: head,
            body: body
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        self.header.to_writer(write)?;
        self.body.to_writer(write)?;

        Ok(())

    }

}

impl DagNode for Block {

    fn version(&self) -> u32 {
        self.clone().header.extract().version
    }

    fn timestamp(&self) -> i64 {
        self.clone().header.extract().timestamp
    }

}

#[cfg(test)]
mod test {

    use std::io::Cursor;

    use core::sig::Hash;
    
    use super::*;

    fn encode_and_decode<T: BinaryComponent>(t: T) -> T {

        let mut c = Cursor::new(Vec::new());
        t.to_writer(&mut c).unwrap();
        let d = c.into_inner();
        println!("{:?}", d);
        T::from_reader(&mut Cursor::new(d)).unwrap()

    }

    #[test]
    fn ck_blockheader_between_blob() {

        let block = BlockHeader {
            version: 42,
            timestamp: 1337,
            segments_merkle_root: Hash::of_slice(&[1, 2, 3]),
            parents: vec![],
        };

        assert_eq!(block, encode_and_decode(block.clone()))

    }

}
