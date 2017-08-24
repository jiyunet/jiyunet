pub mod tangle;

pub struct DecodeError;

// A hash of all of the contents.
pub trait TangleComponent where Self: Sized, Self: Clone {

    fn from_blob(data: &[u8]) -> Result<Self, DecodeError>;
    fn into_blob(&self) -> &[u8];

    fn get_address_hash(&self) -> tangle::Address;

}
