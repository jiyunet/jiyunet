use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};

use dag::Address;
use dag::DagComponent;
use BlobSource;

/// Stores blobs using some directory, with a root specified.
pub struct FsBlobSource {
    root: PathBuf
}

impl FsBlobSource {

    /// Creates a new `FsBlobSource` using the specified datastore root.
    pub fn new(root: PathBuf) -> FsBlobSource {
        FsBlobSource { root: root }
    }

}

impl BlobSource for FsBlobSource {

    fn get(&self, addr: Address) -> Option<Vec<u8>> {

        match fs::File::open(addr_to_path(self.root.clone(), addr)) {
            Ok(mut f) => {
                let mut data = Vec::new();
                match f.read_to_end(&mut data) {
                    Ok(_) => Some(data),
                    Err(_) => None
                }
            },
            Err(_) => None
        }

    }

    fn put(&self, addr: Address, blob: Vec<u8>) -> Result<(), ()> {

        let path = addr_to_path(self.root.clone(), addr);
        if !fs::metadata(path.clone()).is_ok() {
            match fs::File::create(path.clone()) {
                Ok(_) => {}, // Intentionally do nothing.
                Err(_) => return Err(()) // FIXME Make this better.
            }
        }

        match fs::File::open(path) { // Don't need a .clone() as this is the last use.
            Ok(mut f) => match f.write_all(blob.as_slice()) {
                Ok(_) => Ok(()),
                Err(_) => Err(()) // FIXME Make this better.c
            },
            Err(_) => Err(())
        }

    }

}

const BTREE_SPLIT: usize = 4; // sqrt(sizeof(sha256_hash)).  Also not technically for a B-Tree.

fn addr_to_path(root: PathBuf, addr: Address) -> PathBuf {

    let mut path = root.clone();
    let hex = addr.get_hash().into_array();
    path.push(slice_to_hexadecimal(&hex[..BTREE_SPLIT]));
    path.push(slice_to_hexadecimal(&hex[BTREE_SPLIT..]));
    path

}

fn slice_to_hexadecimal(slice: &[u8]) -> String {

    let mut out = String::with_capacity(slice.len() * 2);
    for b in slice {
        for n in vec![(b & 0xf0) >> 4, b & 0x0f] {
            out.push_str(match n {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                10 => "a",
                11 => "b",
                12 => "c",
                13 => "d",
                14 => "e",
                15 => "f",
                _ => ""
            });
        }
    }

    out

}

#[cfg(test)]
mod test {

    use fs;

    #[test]
    fn test_slice_to_hexadecimal_1() {
        assert_eq!(fs::slice_to_hexadecimal(&[0xca, 0xfe, 0xba, 0xbe]), "cafebabe");
    }

    #[test]
    fn test_slice_to_hexadecimal_2() {
        assert_eq!(fs::slice_to_hexadecimal(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
    }

    #[test]
    fn test_slice_to_hexadecimal_3() {
        assert_eq!(fs::slice_to_hexadecimal(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), "0123456789abcdef");
    }

}
