extern crate libjiyunet_dag as dag;
extern crate libjiyunet_db as db;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ValidationError {
    DecodeError(dag::Address), // Problem decoding data.
    NodeNotFound(dag::Address), // Node not found in db, try again later?
    ComponentTooLarge(dag::sig::Hash), // If something is too big to be allowed.
    InsufficientCredits, // Identitiy doesn't have credits for some action.
}

type SignedBlock = dag::comp::Signed<dag::comp::Block>;
type SignedArtifactContainer = dag::comp::Signed<dag::comp::ArtifactContainer>;

enum Addable {
    Block(SignedBlock),
    ArtifactContainer(SignedArtifactContainer)
}

trait Validate {

    fn new(genesis: dag::comp::Signed<dag::comp::Block>) -> Result<Box<Self>, ValidationError>;
    fn include(self, added: Addable) -> Result<Box<Self>, (Box<Self>, ValidationError)>;

}
