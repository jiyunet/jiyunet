use std::collections::HashMap;

use core::Address;
use core::io::BinaryComponent;
use core::sig;
use core::sig::{Fingerprint, ValidationKey};
use core::sig::Signed;

use dag::block;
use dag::segment;

#[derive(Copy, Clone, Eq, PartialEq)]
struct IdentData {
    key: ValidationKey,
    credits: u64
}

type VBlock = Signed<block::Block>;

#[derive(Clone, Eq, PartialEq)]
struct ValdiationState {
    pending: HashMap<Address, VBlock>,
    idents: HashMap<Fingerprint, IdentData>
}

/// Number of blocks that must be accepted before the base is moved up to the next safe block, any
/// parial forks will hold back the "base" until they're resolved.
const ACCEPTANCE_THRESHOLD: u64 = 5;

impl ValdiationState {

    pub fn add_block(&mut self, block: VBlock) -> Result<(), BlockValidationError> {
        self.pending.insert(Address::of_bincomp(&block), block);
        Ok(()) // TODO This isn't correct.  We need to actually check things.
    }

    pub fn find_key(&self, fp: Fingerprint) -> Option<ValidationKey> {
        self.idents.get(&fp).map(|id| id.key)
    }

}

enum BlockValidationError {
    InsufficientCredits,
    SigError(sig::SigVerificationError)
}

type SegmentCost = u64;
const IDENT_COST: SegmentCost = 1000;
const ARTIFACT_PTR_COST: SegmentCost = 50;

fn calc_segment_cost(seg: segment::Segment) -> SegmentCost {
    use dag::segment::SegmentContent::*;
    match seg.content() {
        IdentDecl(_) => IDENT_COST,
        Artifact(ad) => ad.to_blob().len() as SegmentCost, // TODO Make this more mathy.
        ArtifactPointer(_) => ARTIFACT_PTR_COST
    }
}
