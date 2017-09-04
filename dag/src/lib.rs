extern crate byteorder;
extern crate crypto;

pub mod comp;
pub mod sig;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address(sig::Hash);

pub type SignedBlock = comp::Signed<comp::Block>;
pub type SignedArtifactContainer = comp::Signed<comp::ArtifactContainer>;

pub trait DagComponent where Self: Clone {

    // <(the object, bytes consumed), error info>
    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError>;
    fn to_blob(&self) -> Vec<u8>;

    fn get_hash(&self) -> sig::Hash {
        sig::Hash::from_blob(self.to_blob().as_slice())
    }

    fn into_signed(self, kp: sig::Keypair) -> comp::Signed<Self> {
        comp::Signed::new(kp, self)
    }

}

pub trait DagNode where Self: DagComponent {

    fn version(&self) -> u32;
    fn timestamp(&self) -> i64;

}

impl Address {

    pub fn new(hex: [u8; sig::SHA256_WIDTH]) -> Address {
        Address(sig::Hash::new(hex))
    }

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
