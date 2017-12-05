extern crate byteorder;

use std::error;
use std::fmt;
use std::io;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use sig;
use sig::Signed;

/// Type alias for `to_writer`.
pub type WrResult = Result<(), ()>;

/// Defines something that is used to make up the DAG.  Does not have to be a standalone node
/// (see `DagNode`) but does have to be able to have a standard representation as bytes.  We have
/// to do this all by hand because we need to ensure that the exact representation of blocks WILL
/// NOT change without us actually changing it, so we can't risk a dependency slightly changing how
/// it encodes structrues.  We also have a special way of encoding certain things, which can't be
/// easily described to a general-purpose library.
pub trait BinaryComponent where Self: Clone {

    /// Reads input from some kind of byte reader, potentially failing.
    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError>;

    /// Decodes from a slice, potentially failing.
    fn from_slice(s: &[u8]) -> Result<Self, DecodeError> {
        Self::from_reader(&mut Cursor::new(s))
    }

    /// Writes the binary representation of itself to the byte writer, potentially returning the total number of bytes written.
    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult;

    /// Converts the component into its byte representation.  Should not be able to fail.
    fn to_blob(&self) -> Vec<u8> {
        let mut c = Cursor::new(Vec::new());
        self.to_writer(&mut c).expect("Failure encoding struct to Vec.");
        c.into_inner()
    }

    /// Returns the hash of the component, used to create the address of it, if applicable.
    fn get_hash(&self) -> sig::Hash { // TODO Rename to to_hash.
        sig::Hash::of_slice(self.to_blob().as_slice())
    }

    /// Converts the component into the signed version of itself.  This can be done repeatedly as
    /// `Signed<T` is also a component, technically.
    fn into_signed(self, kp: sig::Keypair) -> Signed<Self> {
        Signed::new(kp, self)
    }

}

/// An error in decoding a DagComponent.  Should propagate up the call stack.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecodeError;

impl From<io::Error> for DecodeError {
    fn from(_: io::Error) -> Self {
        DecodeError
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while reading DAG")
    }
}

impl error::Error for DecodeError {
    fn description(&self) -> &str { "a decoding error" }
}

impl BinaryComponent for String {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        let len = read.read_u64::<BigEndian>().map_err(|_| DecodeError)?;
        let mut utf8 = vec![0; len as usize];
        read.read(utf8.as_mut_slice()).map_err(|_| DecodeError)?;
        match String::from_utf8(utf8) {
            Ok(s) => Ok(s),
            Err(_) => Err(DecodeError)
        }
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write_u64::<BigEndian>(self.len() as u64).map_err(|_| ())?;
        write.write(self.as_bytes()).map_err(|_| ())?;
        Ok(())
    }

}

impl<T> BinaryComponent for Option<T> where T: BinaryComponent {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        match read.read_u8().map_err(|_| DecodeError)? {
            0 => Ok(None),
            1 => Ok(Some(T::from_reader(read)?)),
            _ => Err(DecodeError)
        }
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        write.write_u8(if self.is_some() { 1 } else { 0 }).map_err(|_| ())?;
        match self {
            &Some(ref t) => t.to_writer(write)?,
            &None => {}
        }

        Ok(())

    }

}

impl<T> BinaryComponent for Vec<T> where T: BinaryComponent {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let len = read.read_u64::<BigEndian>().map_err(|_| DecodeError)? as usize;

        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(T::from_reader(read)?);
        }

        Ok(v)

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        write.write_u64::<BigEndian>(self.len() as u64).map_err(|_| ())?;
        for e in self {
            e.to_writer(write)?;
        }

        Ok(())

    }

}

impl BinaryComponent for [u8; 64] {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        let mut v = [0; 64];
        read.read(&mut v).map_err(|_| DecodeError)?;
        Ok(v)
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        write.write(self).map_err(|_| ())?;
        Ok(())
    }

}
