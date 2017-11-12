extern crate byteorder;
extern crate crypto;

use byteorder::{ReadBytesExt, WriteBytesExt};

pub mod io;
pub mod sig;

use io::{BinaryComponent, DecodeError, WrResult};
use sig::Hash;

/// Used to directly address something in the DAG.  Just a SHA-256 hash of whatever it is that
/// it's addressing.  Yay for content-addressed objects.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Address(sig::Hash);

impl Address {

    /// Creates a new address, assuming the specified hash data.
    pub fn new(hash: Hash) -> Address {
        Address(hash)
    }

    pub fn from_raw(hex: [u8; sig::SHA256_WIDTH]) -> Address {
        Address::new(Hash::new(hex))
    }

    /// Returns the address of the given blob, assuming that it's a node in a dag.
    pub fn of_slice(blob: &[u8]) -> Address {
        Address(Hash::of_slice(blob))
    }

    pub fn of_bincomp<T: BinaryComponent>(t: &T) -> Address {
        Address::new(t.get_hash())
    }

}

impl BinaryComponent for Address {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        Ok(Address::new(Hash::from_reader(read)?))
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        let &Address(h) = self;
        h.to_writer(write)
    }

}
