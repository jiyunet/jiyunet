extern crate byteorder;
extern crate crypto;

pub mod comp;
pub mod sig;

/// Used to directly address something in the DAG.  Just a SHA-256 hash of whatever it is that
/// it's addressing.  Yay for content-addressed objects.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address(sig::Hash);

/// Simpler way to refer to the actual block on the chain, as they need to be signed.
pub type SignedBlock = comp::Signed<comp::Block>;

/// Simpler way to refer to artifacts stored off-chain, as they *still* need to be signed.
pub type SignedArtifactContainer = comp::Signed<comp::ArtifactContainer>;

/// Defines something that is used to make up the DAG.  Does not have to be a standalone node
/// (see `DagNode`) but does have to be able to have a standard representation as bytes.
pub trait DagComponent where Self: Clone {

    /// Creates a component from a blob (a slice, actually) of bytes.
    ///
    /// `<(the object, bytes consumed), error info>`
    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError>;

    /// Converts the component into its byte representation.  Should not be able to fail.
    fn to_blob(&self) -> Vec<u8>;

    /// Returns the hash of the component, used to create the address of it, if applicable.
    fn get_hash(&self) -> sig::Hash { // TODO Rename to to_hash.
        sig::Hash::from_blob(self.to_blob().as_slice())
    }

    /// Converts the component into the signed version of itself.  This can be done repeatedly as
    /// `Signed<T` is also a component, technically.
    fn into_signed(self, kp: sig::Keypair) -> comp::Signed<Self> {
        comp::Signed::new(kp, self)
    }

}

/// Represents a standalone node on the DAG.  Should always also be a `Signed<T>` something, but
/// this does not *need* to be true.
pub trait DagNode where Self: DagComponent {

    /// Version identifier.  TODO Formalize this.
    fn version(&self) -> u32;

    /// Millisecond UNIX time
    fn timestamp(&self) -> i64;

}

impl Address {

    /// Creates a new address, assuming the specified hash data.
    pub fn new(hex: [u8; sig::SHA256_WIDTH]) -> Address {
        Address(sig::Hash::new(hex))
    }

    /// Returns the address of the given blob, assuming that it's a node in a dag.
    pub fn of(blob: &[u8]) -> Address {
        Address(sig::Hash::from_blob(blob))
    }

}

impl DagComponent for Address {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {
        if blob.len() >= sig::SHA256_WIDTH {

            let mut buf = [0; sig::SHA256_WIDTH];
            for i in 1..sig::SHA256_WIDTH {
                buf[i] = blob[i]; // FIXME Make this *more functional*!
            }

            Ok((Address(sig::Hash::new(buf)), sig::SHA256_WIDTH))

        } else {
            Err(DecodeError)
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        let &Address(h) = self;
        let mut v = Vec::new();
        v.extend_from_slice(&h.into_array());
        v

    }

}

/// An error in decoding a DagComponent.  Should propagate up the call stack.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DecodeError;

impl From<std::io::Error> for DecodeError {
    fn from(_: std::io::Error) -> Self {
        DecodeError
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error while reading DAG")
    }
}

impl std::error::Error for DecodeError {
    fn description(&self) -> &str { "a decoding error" }
}
