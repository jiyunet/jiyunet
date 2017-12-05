extern crate jiyunet_core as core;

extern crate byteorder;
extern crate crypto;

use core::io::BinaryComponent;
use core::sig::Signed;

pub mod comp;

/// Simpler way to refer to the actual block on the chain, as they need to be signed.
pub type SignedBlock = Signed<comp::Block>;

/// Simpler way to refer to artifacts stored off-chain, as they *still* need to be signed.
pub type SignedArtifactContainer = Signed<comp::ArtifactContainer>;

/// Represents a standalone node on the DAG.  Should always also be a `Signed<T>` something, but
/// this does not *need* to be true.
pub trait DagNode: BinaryComponent {

    /// Version identifier.  TODO Formalize this.
    fn version(&self) -> u32;

    /// Millisecond UNIX time
    fn timestamp(&self) -> i64;

}
