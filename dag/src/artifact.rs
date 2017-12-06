use byteorder::*;

use core::io::{BinaryComponent, DecodeError, WrResult};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArtifactData {
    spec: u16, // Artifact code.  Big-endian 0xXXXY, where X is the namespace and Y is the subtype.
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

impl ArtifactData {
    pub fn new(spec: u16, body: Vec<u8>) -> ArtifactData {
        ArtifactData {
            spec: spec,
            body: body
        }
    }
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
        write.write_u16::<BigEndian>(self.spec)?;
        write.write_u64::<BigEndian>(self.body.len() as u64)?;
        write.write_all(self.body.as_slice())?;
        Ok(())
    }

}
