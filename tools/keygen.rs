extern crate jiyunet_core as core;

use std::env;
use std::fs;
use std::path;
use std::io::{self, Read, Write};

use core::io::BinaryComponent;
use core::sig;

fn main() {

    let dest = env::args().nth(1).unwrap_or("jiyu-keypair.bin".into());
    let mut df = fs::File::create(path::PathBuf::from(dest.clone()).as_path())
                            .expect("unable to create destination file.");

    let mut entropy = [0; 1024];
    match io::stdin().read(&mut entropy) {
        Ok(c) => println!("read {} bytes", c),
        Err(e) => println!("error reading input: {}", e)
    }

    let kp = sig::Scheme::Ed25519.generate(&entropy);
    match kp.to_writer(&mut df) {
        Ok(_) => println!("keypair saved to {}", dest),
        Err(_) => println!("unable to write to destination file")
    }

}
