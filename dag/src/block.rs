use byteorder::*; // FIXME

use core::Address;
use core::io::{BinaryComponent, DecodeError, WrResult};
use core::sig::{Hash, Signed};

use segment::*;

use DagNode;

/// Block headers are lightweight peices of information that, when signed, can be passed around
/// easily for coordinating validation between nodes, and for caching actual block data in-memory.
///
/// As of writing, they consume approximately 140 bytes in-memory once parsed, and less than that
/// when serialized.  This isn't counting signature data, which is another ~128 bytes, but that can
/// be discarded once the block is confirmed locally.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlockHeader {

    /// Version identifier.  TODO Formalize this.
    version: u32,

    /// Millisecond UNIX time
    timestamp: i64,

    /// Number of blocks before this one the can be explored by traversing the parents.
    block_height: u64,

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

        let ver = read.read_u32::<BigEndian>()?;
        let ts = read.read_i64::<BigEndian>()?;
        let bh = read.read_u64::<BigEndian>()?;
        let smr = Hash::from_reader(read)?;
        let pars = Vec::<Address>::from_reader(read)?;

        Ok(BlockHeader {
            version: ver,
            timestamp: ts,
            block_height: bh,
            segments_merkle_root: smr,
            parents: pars
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        write.write_u32::<BigEndian>(self.version)?;
        write.write_i64::<BigEndian>(self.timestamp)?;
        write.write_u64::<BigEndian>(self.block_height)?;
        self.segments_merkle_root.to_writer(write)?;
        self.parents.to_writer(write)?;

        Ok(())

    }

}

/// Main type of node on the dag.  Primary unit of time and validation.
///
/// Blocks have a header including their parent information.  They also contain a set of segments
/// that represent the data actually stored in the blocks.  Segments are for the actual mid-layer
/// validation logic.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block(Signed<BlockHeader>, Vec<Signed<Segment>>);

impl Block {

    pub fn get_header(&self) -> &Signed<BlockHeader> {
        &self.0
    }

    pub fn get_segments(&self) -> &Vec<Signed<Segment>> {
        &self.1
    }

}

impl BinaryComponent for Block {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let head = Signed::<BlockHeader>::from_reader(read)?;
        let body = Vec::<Signed<Segment>>::from_reader(read)?;

        Ok(Block(head, body))

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        self.0.to_writer(write)?;
        self.1.to_writer(write)?;

        Ok(())

    }

}

impl DagNode for Block {

    fn version(&self) -> u32 {
        self.clone().0.extract().version
    }

    fn timestamp(&self) -> i64 {
        self.clone().0.extract().timestamp
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
            block_height: 64,
            segments_merkle_root: Hash::of_slice(&[1, 2, 3]),
            parents: vec![],
        };

        assert_eq!(block, encode_and_decode(block.clone()))

    }

}
