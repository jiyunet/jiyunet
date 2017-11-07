use std::collections::HashMap;

use core::Address;
use core::io::BinaryComponent;
use core::sig::{Fingerprint, ValidationKey};

use dag::comp;

#[derive(Copy, Clone)]
struct IdentData {
    key: ValidationKey,
    credits: u64
}

#[derive(Clone)]
struct ValdiationState {
    dangling: Vec<Address>,
    idents: HashMap<Fingerprint, IdentData>
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
