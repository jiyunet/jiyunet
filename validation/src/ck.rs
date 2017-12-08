//! This is the core of the validation code for Jiyunet.  It's highly unfinished, but it will end
//! up looking not unlike the Ethereum validation code.  The first phase is where blocks only have
//! their signatures and fingerprints validated (actual credit checking is ignored), before being
//! added to the full queue, where the rest of the signature data is applied, then the changes to
//! the blockchain state is committed to storage before the next block goes through the full check.
//!
//! It's based somewhat on the Parity validation code:
//! * https://github.com/paritytech/parity/blob/master/ethcore/src/verification/verification.rs

use std::collections::{HashMap, LinkedList};

use core::Address;
use core::io::BinaryComponent;
use core::sig;
use core::sig::{Fingerprint, ValidationKey};
use core::sig::Signed;

use dag::block;
use dag::segment;

use ValidationError;

#[derive(Copy, Clone, Eq, PartialEq)]
struct IdentData {
    key: ValidationKey,
    credits: u64
}

type VBlock = Signed<block::Block>;

#[derive(Clone)]
struct BlockchainState {
    idents: HashMap<Fingerprint, IdentData>,
}

impl BlockchainState {
    fn find_identity(&self, fp: &Fingerprint) -> Option<IdentData> {
        self.idents.get(fp).cloned()
    }
}

#[derive(Clone)]
struct ValdiationState {
    history: LinkedList<(Address, VBlock)>,
    stray_uncles: HashMap<Address, VBlock>,
    data_state: BlockchainState
}

impl ValdiationState {

    pub fn verify_block(&mut self, block: VBlock) -> Result<(), ValidationError> {
        Ok(()) // TODO This isn't correct.  We need to actually check things.
    }

    pub fn find_key(&self, fp: Fingerprint) -> Option<ValidationKey> {
        self.data_state.idents.get(&fp).map(|id| id.key)
    }

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
