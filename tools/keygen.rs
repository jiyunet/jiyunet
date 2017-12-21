extern crate jiyunet_core as core;

#[macro_use] extern crate clap;
extern crate rand;

use std::fs;
use std::path;

use rand::Rng;
use rand::os::OsRng;

use core::io::BinaryComponent;
use core::sig;

fn main() {

    let matches = clap_app!(jiyu_keygen =>
        (version: "0.1.0")
        (author: "treyzania <treyzania@gmail.com>")
        (about: "Generates a Jiyunet keypair.")
        (@arg dest: "File to write to.  Default: jiyu-keypair.bin"))
        .get_matches();

    let dest = matches.value_of("dest").unwrap_or("jiyu-keypair.bin".into());
    let mut df = fs::File::create(path::PathBuf::from(dest.clone()).as_path())
                            .expect("unable to create destination file.");

    let mut seed = [0; 4096];
    let mut rng = match OsRng::new() {
        Ok(r) => r,
        Err(e) => panic!("could not initialize RNG: {}", e)
    };

    rng.fill_bytes(&mut seed);

    // Actually generate the seed.
    let kp = sig::Scheme::Ed25519.generate(&seed);
    match kp.to_writer(&mut df) {
        Ok(_) => println!("keypair saved to {}", dest),
        Err(_) => println!("unable to write to destination file")
    }

    match kp {
        sig::Keypair::Ed25519(k, p) => {
            println!("scheme: ed25519");
            println!("private key: {}", u8_slice_to_string(&k));
            println!("public key: {}", u8_slice_to_string(&p));
        }
    }

}

fn u8_slice_to_string(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(data.len() * 2);
    for i in 0..data.len() {
        write!(&mut s, "{:X}", data[i]).expect("aw shit");
    }
    s
}
