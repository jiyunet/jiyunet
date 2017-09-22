pub mod db;

/// Represents a method of connecting to a remote peer.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ConnectionScheme {

    /// A TCP connection.  The string can be an IP address (IPv4 or IPv6) or a hostname.
    Tcp(String, u16)

}

/// A record in our local copy of the DHT of some peer we can connect to.
///
/// TODO Develop a method of revoking records.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PeerRecord {

    /// The self-identified name of this peer.
    pub name: Option<String>,

    /// The scheme for connecting to the peer to talk the protocol over.
    pub endpoint: ConnectionScheme,

    /// UNIX time in milliseconds when this record should expire.
    pub expiration: u64

}

/// The ping, in milliseconds, between our own node and the other node.
pub type Ping = Option<u32>;
