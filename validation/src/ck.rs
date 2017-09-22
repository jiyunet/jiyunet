extern crate libjiyunet_dag as dag;

use std::collections::HashMap;

use dag::Address;
use dag::DagComponent;
use dag::sig::Fingerprint;

#[derive(Copy, Clone)]
struct IdentData {
    key: dag::sig::ValidationKey,
    credits: u64
}

#[derive(Clone)]
struct ValdiationState {
    dangling: Vec<dag::Address>,
    idents: HashMap<dag::sig::Fingerprint, IdentData>
}

type SegmentCost = u64;
const IDENT_COST: SegmentCost = 1000;
const ARTIFACT_PTR_COST: SegmentCost = 50;

fn calc_segment_cost(seg: dag::comp::Segment) -> SegmentCost {
    use dag::comp::SegmentContent::*;
    match seg.content() {
        IdentDecl(_) => IDENT_COST,
        Artifact(ad) => ad.to_blob().len() as SegmentCost, // TODO Make this more mathy.
        ArtifactPointer(_) => ARTIFACT_PTR_COST
    }
}
