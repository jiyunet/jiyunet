extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;

pub mod fs;
pub mod ipfs;

use core::Address;
use dag::DagNode;

/// Specifies a type that can be used to store and retrieve blobs, given addresses.
pub trait BlobSource {

    /// Returns the blob (in `Vec` form) of the address for this source, if it exists.
    fn get(&self, addr: Address) -> Option<Vec<u8>>;

    /// Stores the blob in the storage, with the specified address, ignoring if the address matches up.
    fn put(&self, addr: Address, blob: Vec<u8>) -> Result<(), ()>; // TODO Make something out of this.

}

/// Used to interact with a `BlobSource`, but converting to and from actual `DagNode`s.
pub struct NodeSource<S> where S: BlobSource {
    source: S
}

/// Some kind of error in finding a node from the datastore.
pub enum NodeGetError {
    NotFound,
    DecodeError(core::io::DecodeError)
}

impl<S> NodeSource<S> where S: BlobSource {

    /// Creates a new `NodeSource` with the given backend.
    pub fn new(src: S) -> NodeSource<S> {
        NodeSource { source: src }
    }

    /// Returns the node with the given address, if possible.
    pub fn get<N: DagNode>(&self, addr: Address) -> Result<N, NodeGetError> {
        match self.source.get(addr) {
            Some(b) => N::from_slice(b.as_slice()).map_err(|e| NodeGetError::DecodeError(e)),
            None => Err(NodeGetError::NotFound)
        }
    }

    /// Stores the node with the address derived from the node.
    pub fn put<N: DagNode>(&self, node: N) -> Result<(), ()> { // FIXME Fix this when it's fixed.
        let blob = node.to_blob();
        self.source.put(Address::of_slice(blob.as_slice()), blob)
    }

}
