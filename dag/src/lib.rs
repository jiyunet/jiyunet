extern crate crypto;

pub mod dag;
pub mod sig;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address(sig::Hash);

pub struct DecodeError;

pub trait DagComponent where Self: Clone {

    // <(the object, bytes consumed), error info>
    fn from_blob(data: &[u8]) -> Result<(Self, usize), DecodeError>;
    fn to_blob(&self) -> Vec<u8>;

    fn get_hash(&self) -> sig::Hash {
        sig::Hash::from_blob(self.to_blob().as_slice())
    }

    fn into_signed(self, kp: sig::Keypair) -> dag::Signed<Self> {
        dag::Signed::new(kp, self)
    }

}

pub trait DagNode where Self: DagComponent {

    fn version(&self) -> u32;
    fn timestamp(&self) -> i64;

}
