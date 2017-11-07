extern crate byteorder;
extern crate crypto;

use byteorder::{ReadBytesExt, WriteBytesExt};

pub mod io;
pub mod sig;

use io::{BinaryComponent, DecodeError};
use sig::Hash;

/// Used to directly address something in the DAG.  Just a SHA-256 hash of whatever it is that
/// it's addressing.  Yay for content-addressed objects.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Address(sig::Hash);

impl Address {

    /// Creates a new address, assuming the specified hash data.
    pub fn new(hex: [u8; sig::SHA256_WIDTH]) -> Address {
        Address(sig::Hash::new(hex))
    }

    /// Returns the address of the given blob, assuming that it's a node in a dag.
    pub fn of_slice(blob: &[u8]) -> Address {
        Address(Hash::of_slice(blob))
    }

}

impl BinaryComponent for Address {

    fn from_reader<R: ReadBytesExt>(read: R) -> Result<Self, DecodeError> {
        unimplemented!();
    }

    fn to_writer<W: WriteBytesExt>(&self, write: W) -> Result<usize, ()> {
        unimplemented!();
    }

}
