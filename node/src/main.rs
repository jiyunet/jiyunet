extern crate jiyunet_dag as dag;
extern crate jiyunet_db as db;
extern crate jiyunet_dht as dht;
extern crate jiyunet_validation as validation;

#[macro_use]
extern crate clap;

fn main() {

    let _matches = clap_app!(myapp =>
            (version: "0.1")
            (author: "The Jiyunet Developers")
            (about: "// TODO Fill this in.")
            (@arg BIND_PORT: -p --bind-port +takes_value "Specifies the port for the daemon to bind to.  Default: 8200")
            (@arg DATA_DIR: -d --data-dir +takes_value "Specifies the block/validation data directory.  Default: /var/lib/jiyunet")
            (@arg CACHE_DIR: -c --cache-dir +takes_value "Specified the artifact cache directory.  Default: /var/cache/jiyunet/artifact")
        ).get_matches();

}
