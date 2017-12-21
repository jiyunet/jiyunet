extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;

#[macro_use] extern crate clap;

use std::fs;
use std::io::Read;

use core::io::BinaryComponent;
use core::sig::Signed;
use dag::artifact;
use dag::segment;

mod util;

fn main() {

    let matches = clap_app!(jiyu_mkart =>
        (version: "0.1.0")
        (author: "treyzania <treyzania@gmail.com>")
        (about: "Packages an file into a signed Jiyunet segment.  Note that the segment is not likely to be valid on the blockchain due to noncing, etc.")
        (@arg src: +required "Source file to package.")
        (@arg dest: +required "Output file.")
        (@arg artifact_type: -a +takes_value "Artifact type.  Default: 0x0000"))
        .get_matches();

    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    let atype = match matches.value_of("artifact_type").map(str::parse) {
        Some(Ok(p)) => p,
        Some(Err(_)) => panic!("unable to parse artifact type as number"),
        None => 0x0000
    };

    // Read the source data, convert to artifact.
    let data = {
        let mut f: fs::File = fs::File::open(src).unwrap();
        let mut v = Vec::new();
        f.read_to_end(&mut v).expect("error reading provided artifact contents");
        v
    };

    let art = artifact::ArtifactData::new(atype, data);
    let seg = segment::Segment::new_artifact_seg(art, util::timestamp());

    // Load the keypair, then sign.
    let kp = util::load_user_keypair().expect("keypair not found");
    let signed_seg = Signed::<segment::Segment>::new(kp, seg);

    // Write the signed artifact segment.
    let mut out = fs::File::create(dest).expect("unable to create destination");
    signed_seg.to_writer(&mut out).expect("unable to write to destination")

}
