#![allow(warnings)]

extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;
extern crate jiyunet_db as db;

pub mod ck;
pub mod io;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ValidationError {
    DecodeError(core::Address), // Problem decoding data.
    NodeNotFound(core::Address), // Node not found in db, try again later?
    ComponentTooLarge(core::sig::Hash), // If something is too big to be allowed.
    InsufficientCredits, // Identitiy doesn't have credits for some action.
}

type SignedBlock = core::sig::Signed<dag::comp::Block>;
type SignedArtifactContainer = core::sig::Signed<dag::comp::ArtifactContainer>;

enum Addable {
    Block(SignedBlock),
    ArtifactContainer(SignedArtifactContainer)
}

trait Validate {

    fn new(genesis: core::sig::Signed<dag::comp::Block>) -> Result<Box<Self>, ValidationError>;
    fn include(self, added: Addable) -> Result<Box<Self>, (Box<Self>, ValidationError)>;

}
