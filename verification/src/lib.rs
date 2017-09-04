extern crate libjiyunet_dag as dag;
extern crate libjiyunet_db as db;

trait VerificationCache {

    fn new() -> Self;

}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VerificationError {
    DecodeError(dag::Address), // Problem decoding data.
    NodeNotFound(dag::Address), // Node not found in db, try again later?
    ComponentTooLarge(dag::sig::Hash), // If something is too big to be allowed.
    InsufficientCredits, // Identitiy doesn't have credits for some action.
}

pub fn verify_signed_block<S: db::BlobSource>(pool: db::NodeSource<S>, block: dag::SignedBlock) -> Result<(), VerificationError> {

    // I feel like I'm doing this totally wrong.
    unimplemented!();

}
