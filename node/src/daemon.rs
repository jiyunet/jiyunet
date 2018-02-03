
use std::io;

use ipfsapi;

use core;

#[allow(dead_code)]
pub struct Jiyud {
    ipfs: IpfsConnection
}

impl Jiyud {

    pub fn new(ipfs: ipfsapi::IpfsApi) -> Jiyud {
        Jiyud {
            ipfs: IpfsConnection {
                api: ipfs
            }
        }
    }

    pub fn run(&self) {
        unimplemented!();
    }

}

struct IpfsConnection {
    api: ipfsapi::IpfsApi
}

pub enum GetError {
    Ipfs,
    NotFound,
    Decode(core::io::DecodeError)
}

impl From<core::io::DecodeError> for GetError {
    fn from(o: core::io::DecodeError) -> Self {
        GetError::Decode(o)
    }
}

impl IpfsConnection {

    fn get_object<T: core::io::BinaryComponent>(&self, addr: core::Address) -> Result<T, GetError> {

        match self.api.cat(addr.to_string().as_ref()) {
            Ok(i) => {
                let mut icr = IpfsCatResult { iter: Box::new(i) };
                Ok(T::from_reader(&mut icr)?)
            },
            Err(_) => Err(GetError::Ipfs)
        }

    }

}

struct IpfsCatResult {
    iter: Box<Iterator<Item=u8>>
}

impl ::std::io::Read for IpfsCatResult {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!(); // This is necessary.
    }

}
