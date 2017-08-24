pub mod dag;

pub struct DecodeError;

// A hash of all of the contents.
pub trait DagComponent where Self: Sized, Self: Clone {

    fn from_blob(data: &[u8]) -> Result<Self, DecodeError>;
    fn into_blob(&self) -> &[u8];

    fn get_address_hash(&self) -> dag::Address;

}
