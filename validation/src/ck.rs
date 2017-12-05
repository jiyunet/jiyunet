use std::collections::HashMap;

use core::Address;
use core::io::BinaryComponent;
use core::sig;
use core::sig::{Fingerprint, ValidationKey};
use core::sig::Signed;

use dag::comp;

#[derive(Copy, Clone, Eq, PartialEq)]
struct IdentData {
    key: ValidationKey,
    credits: u64
}

type VBlock = Signed<comp::Block>;

#[derive(Clone, Eq, PartialEq)]
struct ValdiationState {
    base: (Address, VBlock),
    uncomfirmed: HashMap<Address, VBlock>,
    idents: HashMap<Fingerprint, IdentData>
}

/// Number of blocks that must be accepted before the base is moved up to the next safe block, any
/// parial forks will hold back the "base" until they're resolved.
const ACCEPTANCE_THRESHOLD: u64 = 5;

impl ValdiationState {

    pub fn add_block(&mut self, block: VBlock) -> Result<(), BlockValidationError> {
        unimplemented!(); // TODO Make this just validate signatures for now.
    }

    /// Checks to see if the block is either the last locked-in block or that we have it in our list of pending blocks.
    fn is_address_recent(&self, addr: Address) -> bool {
        self.base.0 == addr || self.uncomfirmed.contains_key(&addr)
    }

    /// Finds the highest height that the given address has in our set of pending blocks.
    fn get_relative_block_height(&self, addr: Address) -> Option<u32> {
        if self.is_address_recent(addr) {
            if self.base.0 == addr {
                Some(0)
            } else {
                match self.uncomfirmed.get(&addr) {
                    Some(b) => b.extract_owned().parents().iter()
                                .map(|p| self.get_relative_block_height(*p))
                                .filter(|h| h.is_some())
                                .map(|v| v.unwrap())
                                .max(),
                    None => None
                }
            }
        } else {
            None
        }
    }

}

enum BlockValidationError {
    InsufficientCredits,
    SigError(sig::SigVerificationError)
}

type SegmentCost = u64;
const IDENT_COST: SegmentCost = 1000;
const ARTIFACT_PTR_COST: SegmentCost = 50;

fn calc_segment_cost(seg: comp::Segment) -> SegmentCost {
    use dag::comp::SegmentContent::*;
    match seg.content() {
        IdentDecl(_) => IDENT_COST,
        Artifact(ad) => ad.to_blob().len() as SegmentCost, // TODO Make this more mathy.
        ArtifactPointer(_) => ARTIFACT_PTR_COST
    }
}
