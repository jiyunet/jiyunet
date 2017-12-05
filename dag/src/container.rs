use byteorder::*;

use core::io::{BinaryComponent, DecodeError, WrResult};
use core::sig::Signed;

use segment::*;

use DagNode;

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
