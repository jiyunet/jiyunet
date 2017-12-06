extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;

use std::env;
use std::fs;
use std::io::Read;

use core::io::BinaryComponent;
use core::sig::Signed;
use dag::artifact;
use dag::segment;

mod util;

fn main() {

    // Lazy argument parsing.
    let src = env::args().nth(1).expect("usage: jiyu-mkart <source> <dest>");
    let dest = env::args().nth(2).expect("usage: jiyu-mkart <source> <dest>");

    // Read the source data, convert to artifact.
    let data = {
        let mut f: fs::File = fs::File::open(src).unwrap();
        let mut v = Vec::new();
        f.read_to_end(&mut v).expect("error reading provided artifact contents");
        v
    };

    let art = artifact::ArtifactData::new(0x0000, data);
    let seg = segment::Segment::new_artifact_seg(art, util::timestamp());

    // Load the keypair, then sign.
    let kp = util::load_user_keypair().expect("keypair not found");
    let signed_seg = Signed::<segment::Segment>::new(kp, seg);

    // Write the signed artifact segment.
    let mut out = fs::File::create(dest).expect("unable to create destination");
    signed_seg.to_writer(&mut out).expect("unable to write to destination")

}
