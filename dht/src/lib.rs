extern crate libjiyunet_core as core;

extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use core::sig;
use core::io::{DecodeError, WrResult};

pub mod db;

/// Represents a method of connecting to a remote peer.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ConnectionScheme {

    /// A TCP connection.  The string can be an IP address (IPv4 or IPv6) or a hostname.  `(host, port)`
    Tcp(String, u16)

}

/// The ping, in milliseconds, between our own node and the other node.
pub type Ping = Option<u32>;

impl core::io::BinaryComponent for ConnectionScheme {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {
        match read.read_u8()? {
            0x00 => {
                let host = String::from_reader(read)?;
                let port = read.read_u16::<BigEndian>()?;
                Ok(ConnectionScheme::Tcp(host, port))
            }
            _ => Err(DecodeError)
        }
    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {

        use self::ConnectionScheme::*;

        write.write_u8(match self {
            &Tcp(_, _) => 0x00
        }).map_err(|_| ())?;

        match self {
            &Tcp(ref host, port) => {
                host.to_writer(write)?;
                write.write_u16::<BigEndian>(port).map_err(|_| ())?;
            }
        }

        Ok(())

    }

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
    pub expiration: u64,

    /// Public key of the owner of this peer record.
    pub pubkey: sig::ValidationKey

}

impl core::io::BinaryComponent for PeerRecord {

    fn from_reader<R: ReadBytesExt>(read: &mut R) -> Result<Self, DecodeError> {

        let n = Option::<String>::from_reader(read)?;
        let ep = ConnectionScheme::from_reader(read)?;
        let expr = read.read_u64::<BigEndian>().map_err(|_| DecodeError)?;
        let pk = sig::ValidationKey::from_reader(read)?;

        Ok(PeerRecord {
            name: n,
            endpoint: ep,
            expiration: expr,
            pubkey: pk
        })

    }

    fn to_writer<W: WriteBytesExt>(&self, write: &mut W) -> WrResult {
        self.name.to_writer(write)?;
        self.endpoint.to_writer(write)?;
        write.write_u64::<BigEndian>(self.expiration).map_err(|_| ())?;
        self.pubkey.to_writer(write)?;
        Ok(())
    }

}

/// Verifies that a signed peer record matches the data it says it should have.
pub fn verify_peer_record(spr: &sig::Signed<PeerRecord>) -> Result<(), sig::SigVerificationError> {
    let pk = spr.extract_owned().pubkey;
    sig::verify_signed(spr, pk)
}
