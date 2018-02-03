#![feature(conservative_impl_trait)]

extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;
extern crate jiyunet_db as db;
extern crate jiyunet_dht as dht;
extern crate jiyunet_validation as validation;

#[macro_use] extern crate clap;
extern crate ipfsapi;

mod daemon;

fn main() {

    let args =
        clap_app!(jiyud =>
            (version: "0.1")
            (author: "The Jiyunet Developers")
            (about: "github.com/jiyunet/jiyunet")
            (@arg IPFSD_ADDR: -a --ipfs-addr +takes_value "Specifies the address of the IPFS daemon.  Default: 127.0.0.1")
            (@arg IPFSD_PORT: -p --ipfs-port +takes_value "Specifies the port to connect to for the IPFS daemon.  Default: 5001")
            (@arg DATA_DIR: -d --data-dir +takes_value "Specifies the block/validation data directory.  Default: /var/lib/jiyunet")
            (@arg CACHE_DIR: -c --cache-dir +takes_value "Specified the artifact cache directory.  Default: /var/cache/jiyunet/artifact")
        ).get_matches();

    let api =
        ipfsapi::IpfsApi::new(
            args.value_of("IPFSD_ADDR").unwrap_or("127.0.0.1"),
            match args.value_of("IPFSD_PORT").map(str::parse) {
                Some(Ok(p)) => p,
                Some(Err(a)) => panic!("invalid port number: {}", a),
                None => 5001
            });

    let daemon = daemon::Jiyud::new(api);
    daemon.run();

}
