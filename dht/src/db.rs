use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

use PeerRecord;
use Ping;

pub struct PeerDatabase {
    peers: HashMap<PeerRecord, Ping>
}

#[derive(Clone)]
pub struct PeerDatabaseRef(Arc<RwLock<PeerDatabase>>);

impl PeerDatabase {

    pub fn new() -> PeerDatabaseRef {
        PeerDatabaseRef(Arc::new(RwLock::new(
            PeerDatabase {
                peers: HashMap::new()
            }
        )))
    }

}
