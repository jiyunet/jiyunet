use std::collections::HashMap;

use core::Address;
use core::io::BinaryComponent;
use core::sig;
use core::sig::{Fingerprint, ValidationKey};
use core::sig::Signed;

use dag::comp;

#[derive(Copy, Clone)]
struct IdentData {
    key: ValidationKey,
    credits: u64
}

type VBlock = Signed<comp::Block>;

#[derive(Clone)]
struct ValdiationState {
    base: (Address, VBlock),
    uncomfirmed: HashMap<Address, VBlock>,
    idents: HashMap<Fingerprint, IdentData>
}

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
                    Some(b) => {
                        let mut max = -1;
                        for p in b.extract_owned().parents() {
                            match self.get_relative_block_height(p).map(|h| h as i32) {
                                Some(h) => if h > max {
                                    max = h;
                                },
                                None => {}
                            }
                        }
                        if max < 0 {
                            None
                        } else {
                            Some(max as u32 + 1)
                        }
                    },
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
