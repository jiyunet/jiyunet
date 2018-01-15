
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use io::{BinaryComponent, DecodeError, WrResult};

impl BinaryComponent for u8 {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        Ok(read.read_u8()?)
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_u8(*self)?;
        Ok(())
    }

}

impl BinaryComponent for u16 {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        Ok(read.read_u16::<BigEndian>()?)
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_u16::<BigEndian>(*self)?;
        Ok(())
    }

}

impl BinaryComponent for u32 {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        Ok(read.read_u32::<BigEndian>()?)
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_u32::<BigEndian>(*self)?;
        Ok(())
    }

}

#[derive(Clone)]
pub struct Blob<L>(Vec<u8>, ::std::marker::PhantomData<L>);
pub type SmallBlob = Blob<u8>;
pub type MediumBlob = Blob<u16>;
pub type BigBlob = Blob<u32>;

impl<L: BinaryComponent + Into<usize> + From<usize>> BinaryComponent for Blob<L> {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        let len = L::from_reader(read)?.into();
        let mut body = vec![0; len];
        read.read(&mut body)?;
        Ok(Blob(body, ::std::marker::PhantomData))
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        let len: L = self.0.len().into();
        BinaryComponent::to_writer(&len, write)?; // this isn't the normal way to do it
        write.write(self.0.as_slice())?;
        Ok(())
    }

}
