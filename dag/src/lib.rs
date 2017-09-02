extern crate byteorder;
extern crate crypto;

pub mod comp;
pub mod sig;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address(sig::Hash);

pub struct DecodeError;

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
