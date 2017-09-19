extern crate libjiyunet_dag as dag;
extern crate libjiyunet_db as db;

use dag::DagComponent;

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

type SegmentCost = u64;

const IDENT_COST: SegmentCost = 1000;
const APTR_COST: SegmentCost = 50;

fn calc_segment_cost(seg: dag::comp::Segment) -> SegmentCost {
    use dag::comp::SegmentContent::*;
    match seg.content() {
        IdentDecl(_) => IDENT_COST,
        Artifact(ad) => ad.to_blob().len() as SegmentCost, // TODO Make this more mathy.
        ArtifactPointer(_) => APTR_COST
    }
}
