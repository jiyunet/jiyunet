extern crate time;

use std::env;
use std::fs;
use std::path;
use std::time::{SystemTime, UNIX_EPOCH};

use core::io::BinaryComponent;
use core::sig;

pub fn load_user_keypair() -> Option<sig::Keypair> {

    let kpp = match env::home_dir() {
        Some(mut p) => {
            p.push(".jiyunet");
            p.push("keypair.bin");
            p
        },
        None => path::PathBuf::from(".")
    };

    let mut f = match fs::File::open(kpp) {
        Ok(o) => o,
        Err(_) => return None
    };

    match sig::Keypair::from_reader(&mut f) {
        Ok(kp) => Some(kp),
        Err(_) => None
    }

}

pub fn timestamp() -> i64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    ((dur.as_secs() * 1000) + ((dur.subsec_nanos() / 1000000) as u64)) as i64
}
