extern crate libjiyunet_dag as dag;

use dag::Address;
use dag::DagNode;

pub trait BlobSource {
    fn get(&self, addr: Address) -> Option<Vec<u8>>;
    fn put(&self, blob: Vec<u8>) -> Result<(), ()>; // TODO Make something out of this.
}

pub struct NodeSource<S> where S: BlobSource {
    source: S
}

pub enum NodeGetError {
    NotFound,
    DecodeError(dag::DecodeError)
}

impl<S> NodeSource<S> where S: BlobSource {

    pub fn new(src: S) -> NodeSource<S> {
        NodeSource { source: src }
    }

    pub fn get<N: DagNode>(&self, addr: Address) -> Result<N, NodeGetError> {
        match self.source.get(addr) {
            Some(b) => match N::from_blob(b.as_slice()) {
                Ok((v, _)) => Ok(v),
                Err(e) => Err(NodeGetError::DecodeError(e))
            },
            None => Err(NodeGetError::NotFound)
        }
    }

    pub fn put<N: DagNode>(&self, node: N) -> Result<(), ()> { // FIXME Fix this when it's fixed.
        self.source.put(node.to_blob())
    }

}
