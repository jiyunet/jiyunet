use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use PeerRecord;
use Ping;

/// In-memory storage of the peers on the network.
struct PeerDatabase {
    peers: HashMap<PeerRecord, Ping>
}

/// More ergonomic way of working with the peer database.
#[derive(Clone)]
pub struct PeerDatabaseHandle(Arc<RwLock<PeerDatabase>>);

#[allow(dead_code)]
impl PeerDatabase {

    pub fn new() -> PeerDatabaseHandle {
        PeerDatabaseHandle(Arc::new(RwLock::new(
            PeerDatabase {
                peers: HashMap::new()
            }
        )))
    }

    fn add_peer(&mut self, peer: PeerRecord) -> Result<(), ()> {

        if !self.peers.contains_key(&peer) {
            self.peers.insert(peer, None);
            Ok(())
        } else {
            Err(())
        }

    }

    fn clean_expired(&mut self, before: u64) -> Vec<PeerRecord> {

        // Find the elements we need to remove, just keep the ones that expire after the specified time.
        let rem: Vec<PeerRecord> = self.peers.keys().filter(|ref v| v.expiration > before).map(|ref v| v.clone().to_owned()).collect();
        for r in rem.clone() {
            self.peers.remove(&r); // Remove each element.
        }

        rem

    }

    fn update_peer_ping(&mut self, peer: PeerRecord, ping: Ping) -> Result<(), ()> {

        if self.peers.contains_key(&peer) {
            self.peers.insert(peer, ping);
            Ok(())
        } else {
            Err(())
        }

    }

}

impl PeerDatabaseHandle {

    /// Adds a new peer to the database, without a known ping set.
    pub fn add_peer(&mut self, p: PeerRecord) -> Result<(), ()> {
        self.0.write().unwrap().add_peer(p)
    }

    /// Cleans any peers that expire before the specified time.
    pub fn clean_expired(&mut self, before: u64) -> Vec<PeerRecord> {
        self.0.write().unwrap().clean_expired(before)
    }

    /// Updates the ping for an already-known peer.  Can be changed to "unset".
    pub fn update_ping(&mut self, peer: PeerRecord, ping: Ping) -> Result<(), ()> {
        self.0.write().unwrap().update_peer_ping(peer, ping)
    }

}
