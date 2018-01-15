extern crate ipfsapi;

use std::sync::*;

struct IpfsBlobSource {
    api: Arc<ipfsapi::IpfsApi>
}

impl IpfsBlobSource {

    pub fn new(api: Arc<ipfsapi::IpfsApi>) -> IpfsBlobSource {
        IpfsBlobSource {
            api: api
        }
    }

}
