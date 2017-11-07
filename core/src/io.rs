extern crate byteorder;

use std::error;
use std::fmt;
use std::io;
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};

use sig;
use sig::Signed;

/// Type alias for `to_writer`.
pub type WrResult = Result<(), ()>;

/// Defines something that is used to make up the DAG.  Does not have to be a standalone node
/// (see `DagNode`) but does have to be able to have a standard representation as bytes.
pub trait BinaryComponent where Self: Clone {

    /// Reads input from some kind of byte reader, potentially failing.
    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError>;

    /// Decodes from a slice, potentially failing.
    fn from_slice(s: &[u8]) -> Result<Self, DecodeError> {
        Self::from_reader(Cursor::new(s))
    }

    /// Writes the binary representation of itself to the byte writer, potentially returning the total number of bytes written.
    fn to_writer<W: WriteBytesExt>(&self, write: W) -> WrResult;

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
