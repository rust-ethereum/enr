//! # Ethereum Node Record (ENR)
//!
//! This crate contains an implementation of an Ethereum Node Record (ENR) as specified by
//! [EIP-778](https://eips.ethereum.org/EIPS/eip-778) extended to allow for the use of ed25519 keys.
//!
//! An ENR is a signed, key-value record which has an associated [`NodeId`] (a 32-byte identifier).
//! Updating/modifying an ENR requires an [`EnrKey`] in order to re-sign the record with the
//! associated key-pair.
//!
//! ENR's are identified by their sequence number. When updating an ENR, the sequence number is
//! increased.
//!
//! Different identity schemes can be used to define the node id and signatures. Currently only the
//! "v4" identity is supported and is set by default.
//!
//! ## Signing Algorithms
//!
//! User's wishing to implement their own singing algorithms simply need to
//! implement the [`EnrKey`] trait and apply it to an [`Enr`].
//!
//! By default, `libsecp256k1::SecretKey` implement [`EnrKey`] and can be used to sign and
//! verify ENR records. This library also implements [`EnrKey`] for `ed25519_dalek::Keypair` via the `ed25519`
//! feature flag.
//!
//! Furthermore, a [`CombinedKey`] is provided if the `ed25519` feature flag is set, which provides an
//! ENR type that can support both `secp256k1` and `ed25519` signed ENR records. Examples of the
//! use of each of these key types is given below.
//!
//! ## Features
//!
//! This crate supports a number of features.
//!
//! - `serde`: Allows for serde serialization and deserialization for ENRs.
//! - `ed25519`: Provides support for `ed25519_dalek` keypair types.
//! - `rust-secp256k1`: Uses `c-secp256k1` for secp256k1 keys.
//!
//! These can be enabled via adding the feature flag in your `Cargo.toml`
//!
//! ```toml
//! enr = { version = "*", features = ["serde","ed25519"] }
//! ```
//!
//! ## Examples
//!
//! To build an ENR, an [`EnrBuilder`] is provided.
//!
//! ### Building an ENR with the default `secp256k1` key type
//!
//! ```rust
//! use enr::{EnrBuilder, secp256k1};
//! use std::net::Ipv4Addr;
//! use rand::thread_rng;
//!
//! // generate a random secp256k1 key
//! let mut rng = thread_rng();
//! let key = secp256k1::SecretKey::random(&mut rng);
//!
//! let ip = Ipv4Addr::new(192,168,0,1);
//! let enr = EnrBuilder::new("v4").ip(ip.into()).tcp(8000).build(&key).unwrap();
//!
//! assert_eq!(enr.ip(), Some("192.168.0.1".parse().unwrap()));
//! assert_eq!(enr.id(), Some("v4".into()));
//! ```
//!
//! ### Building an ENR with the `CombinedKey` type (support for multiple signing
//! algorithms).
//!
//! Note the `ed25519` feature flag must be set. This makes use of the
//! [`EnrBuilder`] struct.
//! ```rust
//! # #[cfg(feature = "ed25519")] {
//! use enr::{EnrBuilder, CombinedKey};
//! use std::net::Ipv4Addr;
//!
//! // create a new secp256k1 key
//! let key = CombinedKey::generate_secp256k1();
//!
//! // or create a new ed25519 key
//! let key = CombinedKey::generate_ed25519();
//!
//! let ip = Ipv4Addr::new(192,168,0,1);
//! let enr = EnrBuilder::new("v4").ip(ip.into()).tcp(8000).build(&key).unwrap();
//!
//! assert_eq!(enr.ip(), Some("192.168.0.1".parse().unwrap()));
//! assert_eq!(enr.id(), Some("v4".into()));
//! # }
//! ```
//!
//! ### Modifying an [`Enr`]
//!
//! ENR fields can be added and modified using the getters/setters on [`Enr`]. A custom field
//! can be added using [`insert`] and retrieved with [`get`].
//!
//! ```rust
//! use enr::{EnrBuilder, secp256k1::SecretKey, Enr};
//! use std::net::Ipv4Addr;
//! use rand::thread_rng;
//!
//! // specify the type of ENR
//! type DefaultEnr = Enr<secp256k1::SecretKey>;
//!
//! // generate a random secp256k1 key
//! let mut rng = thread_rng();
//! let key = SecretKey::random(&mut rng);
//!
//! let ip = Ipv4Addr::new(192,168,0,1);
//! let mut enr = EnrBuilder::new("v4").ip(ip.into()).tcp(8000).build(&key).unwrap();
//!
//! enr.set_tcp(8001, &key);
//! // set a custom key
//! enr.insert("custom_key", vec![0,0,1], &key);
//!
//! // encode to base64
//! let base_64_string = enr.to_base64();
//!
//! // decode from base64
//! let decoded_enr: DefaultEnr = base_64_string.parse().unwrap();
//!
//! assert_eq!(decoded_enr.ip(), Some("192.168.0.1".parse().unwrap()));
//! assert_eq!(decoded_enr.id(), Some("v4".into()));
//! assert_eq!(decoded_enr.tcp(), Some(8001));
//! assert_eq!(decoded_enr.get("custom_key"), Some(&vec![0,0,1]));
//! ```
//!
//! ### Encoding/Decoding ENR's of various key types
//!
//! ```rust
//! # #[cfg(feature = "ed25519")] {
//! use enr::{EnrBuilder, secp256k1::SecretKey, Enr, ed25519_dalek::Keypair, CombinedKey};
//! use std::net::Ipv4Addr;
//! use rand::thread_rng;
//! use rand::Rng;
//!
//! // generate a random secp256k1 key
//! let mut rng = thread_rng();
//! let key = SecretKey::random(&mut rng);
//! let ip = Ipv4Addr::new(192,168,0,1);
//! let enr_secp256k1 = EnrBuilder::new("v4").ip(ip.into()).tcp(8000).build(&key).unwrap();
//!
//! // encode to base64
//! let base64_string_secp256k1 = enr_secp256k1.to_base64();
//!
//! // generate a random ed25519 key
//! let key = Keypair::generate(&mut rng);
//! let enr_ed25519 = EnrBuilder::new("v4").ip(ip.into()).tcp(8000).build(&key).unwrap();
//!
//! // encode to base64
//! let base64_string_ed25519 = enr_ed25519.to_base64();
//!
//! // decode base64 strings of varying key types
//! // decode the secp256k1 with default Enr
//! let decoded_enr_secp256k1: Enr<secp256k1::SecretKey> = base64_string_secp256k1.parse().unwrap();
//! // decode ed25519 ENRs
//! let decoded_enr_ed25519: Enr<ed25519_dalek::Keypair> = base64_string_ed25519.parse().unwrap();
//!
//! // use the combined key to be able to decode either
//! let decoded_enr: Enr<CombinedKey> = base64_string_secp256k1.parse().unwrap();
//! let decoded_enr: Enr<CombinedKey> = base64_string_ed25519.parse().unwrap();
//! # }
//! ```
//!
//!
//! [`CombinedKey`]: enum.CombinedKey.html
//! [`EnrKey`]: trait.EnrKey.html
//! [`Enr`]: struct.EnrBase.html
//! [`EnrBuilder`]: struct.EnrBuilderBase.html
//! [`NodeId`]: struct.NodeId.html
//! [`insert`]: struct.Enr.html#method.insert
//! [`get`]: struct.Enr.html#method.get

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

mod builder;
mod keys;
mod node_id;

use log::debug;
use rlp::{DecoderError, Rlp, RlpStream};
use std::collections::BTreeMap;
use tiny_keccak::{Hasher, Keccak};

#[cfg(feature = "serde")]
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

pub use builder::EnrBuilder;

#[cfg(feature = "rust-secp256k1")]
pub use keys::c_secp256k1;
#[cfg(feature = "libsecp256k1")]
pub use keys::secp256k1;
#[cfg(feature = "ed25519")]
pub use keys::{ed25519_dalek, CombinedKey, CombinedPublicKey};
pub use keys::{EnrKey, EnrPublicKey};
pub use node_id::NodeId;
use std::marker::PhantomData;

const MAX_ENR_SIZE: usize = 300;

/// The ENR, allowing for arbitrary signing algorithms. The default signing algorithm is
/// `secp256k1` using the `libsecp256k1` library.
///
/// This struct will always have a valid signature, known public key type, sequence number and `NodeId`. All other parameters are variable/optional.
#[derive(Eq)]
pub struct Enr<K: EnrKey> {
    /// ENR sequence number.
    seq: u64,

    /// The `NodeId` of the ENR record.
    node_id: NodeId,

    /// Key-value contents of the ENR. A BTreeMap is used to get the keys in sorted order, which is
    /// important for verifying the signature of the ENR.
    content: BTreeMap<String, Vec<u8>>,

    /// The signature of the ENR record, stored as bytes.
    signature: Vec<u8>,

    /// Marker to pin the generic.
    phantom: PhantomData<K>,
}

impl<K: EnrKey> Enr<K> {
    // getters //

    /// The `NodeId` for the record.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// The current sequence number of the ENR record.
    #[must_use]
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Reads a custom key from the record if it exists.
    pub fn get(&self, key: impl Into<String>) -> Option<&Vec<u8>> {
        self.content.get(&key.into())
    }

    /// Returns an iterator over all key/value pairs in the ENR.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<u8>)> {
        self.content.iter()
    }

    /// Returns the IPv4 address of the ENR record if it is defined.
    #[must_use]
    pub fn ip(&self) -> Option<Ipv4Addr> {
        if let Some(ip_bytes) = self.content.get("ip") {
            return match ip_bytes.len() {
                4 => {
                    let mut ip = [0_u8; 4];
                    ip.copy_from_slice(ip_bytes);
                    Some(Ipv4Addr::from(ip))
                }
                _ => None,
            };
        }
        None
    }

    /// Returns the IPv6 address of the ENR record if it is defined.
    #[must_use]
    pub fn ip6(&self) -> Option<Ipv6Addr> {
        if let Some(ip_bytes) = self.content.get("ip6") {
            return match ip_bytes.len() {
                16 => {
                    let mut ip = [0_u8; 16];
                    ip.copy_from_slice(ip_bytes);
                    Some(Ipv6Addr::from(ip))
                }
                _ => None,
            };
        }
        None
    }

    /// The `id` of ENR record if it is defined.
    #[must_use]
    pub fn id(&self) -> Option<String> {
        if let Some(id_bytes) = self.content.get("id") {
            return Some(String::from_utf8_lossy(id_bytes).to_string());
        }
        None
    }

    /// The TCP port of ENR record if it is defined.
    #[must_use]
    pub fn tcp(&self) -> Option<u16> {
        if let Some(tcp_bytes) = self.content.get("tcp") {
            if tcp_bytes.len() <= 2 {
                let mut tcp = [0_u8; 2];
                tcp[2 - tcp_bytes.len()..].copy_from_slice(tcp_bytes);
                return Some(u16::from_be_bytes(tcp));
            }
        }
        None
    }

    /// The IPv6-specific TCP port of ENR record if it is defined.
    #[must_use]
    pub fn tcp6(&self) -> Option<u16> {
        if let Some(tcp_bytes) = self.content.get("tcp6") {
            if tcp_bytes.len() <= 2 {
                let mut tcp = [0_u8; 2];
                tcp[2 - tcp_bytes.len()..].copy_from_slice(tcp_bytes);
                return Some(u16::from_be_bytes(tcp));
            }
        }
        None
    }

    /// The UDP port of ENR record if it is defined.
    #[must_use]
    pub fn udp(&self) -> Option<u16> {
        if let Some(udp_bytes) = self.content.get("udp") {
            if udp_bytes.len() <= 2 {
                let mut udp = [0_u8; 2];
                udp[2 - udp_bytes.len()..].copy_from_slice(udp_bytes);
                return Some(u16::from_be_bytes(udp));
            }
        }
        None
    }

    /// The IPv6-specific UDP port of ENR record if it is defined.
    #[must_use]
    pub fn udp6(&self) -> Option<u16> {
        if let Some(udp_bytes) = self.content.get("udp6") {
            if udp_bytes.len() <= 2 {
                let mut udp = [0_u8; 2];
                udp[2 - udp_bytes.len()..].copy_from_slice(udp_bytes);
                return Some(u16::from_be_bytes(udp));
            }
        }
        None
    }

    /// Provides a socket (based on the UDP port), if the IP and UDP fields are specified.
    #[must_use]
    pub fn udp_socket(&self) -> Option<SocketAddr> {
        if let Some(ip) = self.ip() {
            if let Some(udp) = self.udp() {
                return Some(SocketAddr::new(IpAddr::V4(ip), udp));
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(udp6) = self.udp6() {
                return Some(SocketAddr::new(IpAddr::V6(ip6), udp6));
            }
        }
        None
    }

    /// Provides a socket (based on the TCP port), if the IP and UDP fields are specified.
    #[must_use]
    pub fn tcp_socket(&self) -> Option<SocketAddr> {
        if let Some(ip) = self.ip() {
            if let Some(tcp) = self.tcp() {
                return Some(SocketAddr::new(IpAddr::V4(ip), tcp));
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(tcp6) = self.tcp6() {
                return Some(SocketAddr::new(IpAddr::V6(ip6), tcp6));
            }
        }
        None
    }

    /// The signature of the ENR record.
    #[must_use]
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Returns the public key of the ENR record.
    #[must_use]
    pub fn public_key(&self) -> K::PublicKey {
        K::enr_to_public(&self.content).expect("ENR's can only be created with supported keys")
    }

    /// Verify the signature of the ENR record.
    #[must_use]
    pub fn verify(&self) -> bool {
        let pubkey = self.public_key();
        match self.id() {
            Some(ref id) if id == "v4" => pubkey.verify_v4(&self.rlp_content(), &self.signature),
            // unsupported identity schemes
            _ => false,
        }
    }

    /// RLP encodes the ENR into a byte array.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut s = RlpStream::new();
        s.append(self);
        s.drain()
    }

    /// Provides the URL-safe base64 encoded "text" version of the ENR prefixed by "enr:".
    #[must_use]
    pub fn to_base64(&self) -> String {
        let hex = base64::encode_config(&self.encode(), base64::URL_SAFE_NO_PAD);
        format!("enr:{}", hex)
    }

    /// Returns the current size of the ENR.
    #[must_use]
    pub fn size(&self) -> usize {
        self.rlp_content().len()
    }

    // Setters //

    /// Allows setting the sequence number to an arbitrary value.
    pub fn set_seq(&mut self, seq: u64, key: &K) -> Result<(), EnrError> {
        self.seq = seq;

        // sign the record
        self.sign(key)?;

        // update the node id
        self.node_id = NodeId::from(key.public());

        // check the size of the record
        if self.size() > MAX_ENR_SIZE {
            return Err(EnrError::ExceedsMaxSize);
        }

        Ok(())
    }

    /// Adds or modifies a key/value to the ENR record. A `EnrKey` is required to re-sign the record once
    /// modified.
    ///
    /// Returns the previous value in the record if it exists.
    pub fn insert(
        &mut self,
        key: &str,
        value: Vec<u8>,
        enr_key: &K,
    ) -> Result<Option<Vec<u8>>, EnrError> {
        // currently only support "v4" identity schemes
        if key == "id" && value != b"v4" {
            return Err(EnrError::UnsupportedIdentityScheme);
        }

        let previous_value = self.content.insert(key.into(), value);
        // add the new public key
        let public_key = enr_key.public();
        let previous_key = self
            .content
            .insert(public_key.enr_key(), public_key.encode());

        // check the size of the record
        if self.size() > MAX_ENR_SIZE {
            // if the size of the record is too large, revert and error
            // revert the public key
            if let Some(key) = previous_key {
                self.content.insert(public_key.enr_key(), key);
            } else {
                self.content.remove(&public_key.enr_key());
            }
            // revert the content
            if let Some(prev_value) = previous_value {
                self.content.insert(key.into(), prev_value);
            } else {
                self.content.remove(key);
            }
            return Err(EnrError::ExceedsMaxSize);
        }
        // increment the sequence number
        self.seq = self
            .seq
            .checked_add(1)
            .ok_or_else(|| EnrError::SequenceNumberTooHigh)?;

        // sign the record
        self.sign(enr_key)?;

        // update the node id
        self.node_id = NodeId::from(enr_key.public());

        if self.size() > MAX_ENR_SIZE {
            // incase the signature size changes, inform the user the size has exceeded the maximum
            return Err(EnrError::ExceedsMaxSize);
        }

        Ok(previous_value)
    }

    /// Sets the `ip` field of the ENR. Returns any pre-existing IP address in the record.
    pub fn set_ip(&mut self, ip: IpAddr, key: &K) -> Result<Option<IpAddr>, EnrError> {
        match ip {
            IpAddr::V4(addr) => {
                let prev_value = self.insert("ip", addr.octets().to_vec(), key)?;
                if let Some(bytes) = prev_value {
                    if bytes.len() == 4 {
                        let mut v = [0_u8; 4];
                        v.copy_from_slice(&bytes);
                        return Ok(Some(IpAddr::V4(Ipv4Addr::from(v))));
                    }
                }
            }
            IpAddr::V6(addr) => {
                let prev_value = self.insert("ip6", addr.octets().to_vec(), key)?;
                if let Some(bytes) = prev_value {
                    if bytes.len() == 16 {
                        let mut v = [0_u8; 16];
                        v.copy_from_slice(&bytes);
                        return Ok(Some(IpAddr::V6(Ipv6Addr::from(v))));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Sets the `udp` field of the ENR. Returns any pre-existing UDP port in the record.
    pub fn set_udp(&mut self, udp: u16, key: &K) -> Result<Option<u16>, EnrError> {
        if let Some(udp_bytes) = self.insert("udp", udp.to_be_bytes().to_vec(), key)? {
            if udp_bytes.len() <= 2 {
                let mut v = [0_u8; 2];
                v[2 - udp_bytes.len()..].copy_from_slice(&udp_bytes);
                return Ok(Some(u16::from_be_bytes(v)));
            }
        }
        Ok(None)
    }

    /// Sets the `udp6` field of the ENR. Returns any pre-existing UDP port in the record.
    pub fn set_udp6(&mut self, udp: u16, key: &K) -> Result<Option<u16>, EnrError> {
        if let Some(udp_bytes) = self.insert("udp6", udp.to_be_bytes().to_vec(), key)? {
            if udp_bytes.len() <= 2 {
                let mut v = [0_u8; 2];
                v[2 - udp_bytes.len()..].copy_from_slice(&udp_bytes);
                return Ok(Some(u16::from_be_bytes(v)));
            }
        }
        Ok(None)
    }

    /// Sets the `tcp` field of the ENR. Returns any pre-existing tcp port in the record.
    pub fn set_tcp(&mut self, tcp: u16, key: &K) -> Result<Option<u16>, EnrError> {
        if let Some(tcp_bytes) = self.insert("tcp", tcp.to_be_bytes().to_vec(), key)? {
            if tcp_bytes.len() <= 2 {
                let mut v = [0_u8; 2];
                v[2 - tcp_bytes.len()..].copy_from_slice(&tcp_bytes);
                return Ok(Some(u16::from_be_bytes(v)));
            }
        }
        Ok(None)
    }

    /// Sets the `tcp6` field of the ENR. Returns any pre-existing tcp6 port in the record.
    pub fn set_tcp6(&mut self, tcp: u16, key: &K) -> Result<Option<u16>, EnrError> {
        if let Some(tcp_bytes) = self.insert("tcp6", tcp.to_be_bytes().to_vec(), key)? {
            if tcp_bytes.len() <= 2 {
                let mut v = [0_u8; 2];
                v[2 - tcp_bytes.len()..].copy_from_slice(&tcp_bytes);
                return Ok(Some(u16::from_be_bytes(v)));
            }
        }
        Ok(None)
    }

    /// Sets the IP and UDP port in a single update with a single increment in sequence number.
    pub fn set_udp_socket(&mut self, socket: SocketAddr, key: &K) -> Result<(), EnrError> {
        self.set_socket(socket, key, false)
    }

    /// Sets the IP and TCP port in a single update with a single increment in sequence number.
    pub fn set_tcp_socket(&mut self, socket: SocketAddr, key: &K) -> Result<(), EnrError> {
        self.set_socket(socket, key, true)
    }

    /// Helper function for `set_tcp_socket()` and `set_udp_socket`.
    fn set_socket(&mut self, socket: SocketAddr, key: &K, is_tcp: bool) -> Result<(), EnrError> {
        let (port_string, port_v6_string): (String, String) = if is_tcp {
            ("tcp".into(), "tcp6".into())
        } else {
            ("udp".into(), "udp6".into())
        };

        let (prev_ip, prev_port) = match socket.ip() {
            IpAddr::V4(addr) => (
                self.content.insert("ip".into(), addr.octets().to_vec()),
                self.content
                    .insert(port_string.clone(), socket.port().to_be_bytes().to_vec()),
            ),
            IpAddr::V6(addr) => (
                self.content.insert("ip6".into(), addr.octets().to_vec()),
                self.content
                    .insert(port_v6_string.clone(), socket.port().to_be_bytes().to_vec()),
            ),
        };

        let public_key = key.public();
        let previous_key = self
            .content
            .insert(public_key.enr_key(), public_key.encode());

        // check the size and revert on failure
        if self.size() > MAX_ENR_SIZE {
            // if the size of the record is too large, revert and error
            // revert the public key
            if let Some(key) = previous_key {
                self.content.insert(public_key.enr_key(), key);
            } else {
                self.content.remove(&public_key.enr_key());
            }
            // revert the content
            match socket.ip() {
                IpAddr::V4(_) => {
                    if let Some(ip) = prev_ip {
                        self.content.insert("ip".into(), ip);
                    } else {
                        self.content.remove(&String::from("ip"));
                    }
                    if let Some(udp) = prev_port {
                        self.content.insert(port_string, udp);
                    } else {
                        self.content.remove(&port_string);
                    }
                }
                IpAddr::V6(_) => {
                    if let Some(ip) = prev_ip {
                        self.content.insert("ip6".into(), ip);
                    } else {
                        self.content.remove(&String::from("ip6"));
                    }
                    if let Some(udp) = prev_port {
                        self.content.insert(port_v6_string, udp);
                    } else {
                        self.content.remove(&port_v6_string);
                    }
                }
            }
            return Err(EnrError::ExceedsMaxSize);
        }

        // increment the sequence number
        self.seq = self
            .seq
            .checked_add(1)
            .ok_or_else(|| EnrError::SequenceNumberTooHigh)?;

        // sign the record
        self.sign(key)?;

        // update the node id
        self.node_id = NodeId::from(key.public());

        Ok(())
    }

    /// Sets a new public key for the record.
    pub fn set_public_key(&mut self, public_key: &K::PublicKey, key: &K) -> Result<(), EnrError> {
        self.insert(&public_key.enr_key(), public_key.encode(), key)
            .map(|_| {})
    }

    // Private Functions //

    /// Evaluates the RLP-encoding of the content of the ENR record.
    fn rlp_content(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        stream.begin_list(self.content.len() * 2 + 1);
        stream.append(&self.seq);
        for (k, v) in &self.content {
            stream.append(k);
            stream.append(v);
        }
        stream.drain()
    }

    /// Signs the ENR record based on the identity scheme. Currently only "v4" is supported.
    fn sign(&mut self, key: &K) -> Result<(), EnrError> {
        self.signature = {
            match self.id() {
                Some(ref id) if id == "v4" => key
                    .sign_v4(&self.rlp_content())
                    .map_err(|_| EnrError::SigningError)?,
                // other identity schemes are unsupported
                _ => return Err(EnrError::SigningError),
            }
        };
        Ok(())
    }
}

// traits //

impl<K: EnrKey> Clone for Enr<K> {
    fn clone(&self) -> Self {
        Self {
            seq: self.seq,
            node_id: self.node_id,
            content: self.content.clone(),
            signature: self.signature.clone(),
            phantom: self.phantom,
        }
    }
}

impl<K: EnrKey> PartialEq for Enr<K> {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
            && self.node_id == other.node_id
            && self.content == other.content
            && self.signature == other.signature
    }
}

impl<K: EnrKey> std::fmt::Display for Enr<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ENR: NodeId: {}, Socket: {:?}",
            self.node_id(),
            self.udp_socket()
        )
    }
}

impl<K: EnrKey> std::fmt::Debug for Enr<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}

/// Convert a URL-SAFE base64 encoded ENR into an ENR.
impl<K: EnrKey> FromStr for Enr<K> {
    type Err = String;

    fn from_str(base64_string: &str) -> Result<Self, Self::Err> {
        if base64_string.len() < 4 {
            return Err("Invalid ENR string".to_string());
        }
        // support both enr prefix and not
        let mut decode_string = base64_string;
        if base64_string.starts_with("enr:") {
            decode_string = decode_string
                .get(4..)
                .ok_or_else(|| "Invalid ENR string".to_string())?;
        }
        let bytes = base64::decode_config(decode_string, base64::URL_SAFE_NO_PAD)
            .map_err(|e| format!("Invalid base64 encoding: {:?}", e))?;
        rlp::decode(&bytes).map_err(|e| format!("Invalid ENR: {:?}", e))
    }
}

#[cfg(any(feature = "serde", doc))]
impl<K: EnrKey> Serialize for Enr<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_base64())
    }
}

#[cfg(any(feature = "serde", doc))]
impl<'de, K: EnrKey> Deserialize<'de> for Enr<K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(D::Error::custom)
    }
}

impl<K: EnrKey> rlp::Encodable for Enr<K> {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(self.content.len() * 2 + 2);
        s.append(&self.signature);
        s.append(&self.seq);
        // must use rlp_content to preserve ordering.
        for (k, v) in &self.content {
            s.append(k);
            s.append(v);
        }
    }
}

impl<K: EnrKey> rlp::Decodable for Enr<K> {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if !rlp.is_list() {
            debug!("Failed to decode ENR. Not an RLP list: {}", rlp);
            return Err(DecoderError::RlpExpectedToBeList);
        }

        let mut decoded_list = rlp.as_list::<Vec<u8>>().map_err(|_| {
            debug!("Could not decode content: {}", rlp);
            DecoderError::Custom("List decode fail")
        })?;

        if decoded_list.is_empty() || decoded_list.len() % 2 != 0 {
            debug!("Failed to decode ENR. List size is not a multiple of 2.");
            return Err(DecoderError::Custom("List not a multiple of two"));
        }

        let signature = decoded_list.remove(0);
        let seq_bytes = decoded_list.remove(0);

        if seq_bytes.len() > 8 {
            debug!("Failed to decode ENR. Sequence number is not a u64.");
            return Err(DecoderError::Custom("Invalid Sequence number"));
        }

        // build u64 from big endian vec<u8>
        let mut seq: [u8; 8] = [0; 8];
        seq[8 - seq_bytes.len()..].copy_from_slice(&seq_bytes);
        let seq = u64::from_be_bytes(seq);

        let mut content = BTreeMap::new();
        let mut prev: Option<String> = None;
        for _ in 0..decoded_list.len() / 2 {
            let key = decoded_list.remove(0);
            let value = decoded_list.remove(0);

            let key = String::from_utf8_lossy(&key);
            // TODO: add tests for this error case
            if prev.is_some() && prev >= Some(key.to_string()) {
                return Err(DecoderError::Custom("Unsorted keys"));
            }
            prev = Some(key.to_string());
            content.insert(key.to_string(), value);
        }

        // verify we know the signature type
        let public_key = K::enr_to_public(&content)?;

        // calculate the node id
        let node_id = NodeId::from(public_key);

        let enr = Self {
            seq,
            node_id,
            signature,
            content,
            phantom: PhantomData,
        };

        // verify the signature before returning
        // if the public key is of an unknown type, this will fail.
        // An ENR record will always have a valid public-key and therefore node-id
        if !enr.verify() {
            return Err(DecoderError::Custom("Invalid Signature"));
        }
        Ok(enr)
    }
}

#[derive(Clone, Debug)]
/// An error type for handling various ENR operations.
pub enum EnrError {
    /// The ENR is too large.
    ExceedsMaxSize,
    /// The sequence number is too large.
    SequenceNumberTooHigh,
    /// There was an error with signing an ENR record.
    SigningError,
    /// The identity scheme is not supported.
    UnsupportedIdentityScheme,
}

pub(crate) fn digest(b: &[u8]) -> [u8; 32] {
    let mut output = [0_u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(b);
    hasher.finalize(&mut output);
    output
}

#[cfg(test)]
#[cfg(feature = "libsecp256k1")]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    type DefaultEnr = Enr<secp256k1::SecretKey>;

    #[cfg(feature = "libsecp256k1")]
    #[test]
    fn check_test_vector() {
        let valid_record = hex::decode("f884b8407098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c01826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765f").unwrap();
        let signature = hex::decode("7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c").unwrap();
        let expected_pubkey =
            hex::decode("03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138")
                .unwrap();

        let enr = rlp::decode::<DefaultEnr>(&valid_record).unwrap();

        let pubkey = enr.public_key().encode();

        assert_eq!(enr.ip(), Some(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(enr.id(), Some(String::from("v4")));
        assert_eq!(enr.udp(), Some(30303));
        assert_eq!(enr.tcp(), None);
        assert_eq!(enr.signature(), &signature[..]);
        assert_eq!(pubkey, expected_pubkey);
        assert!(enr.verify());
    }

    #[cfg(feature = "libsecp256k1")]
    #[test]
    fn check_test_vector_2() {
        let text = "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8";
        let signature = hex::decode("7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c").unwrap();
        let expected_pubkey =
            hex::decode("03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138")
                .unwrap();
        let expected_node_id =
            hex::decode("a448f24c6d18e575453db13171562b71999873db5b286df957af199ec94617f7")
                .unwrap();

        let enr = text.parse::<DefaultEnr>().unwrap();
        let pubkey = enr.public_key().encode();
        assert_eq!(enr.ip(), Some(Ipv4Addr::new(127, 0, 0, 1)));
        dbg!("here");
        assert_eq!(enr.ip6(), None);
        dbg!("here");
        assert_eq!(enr.id(), Some(String::from("v4")));
        assert_eq!(enr.udp(), Some(30303));
        assert_eq!(enr.udp6(), None);
        assert_eq!(enr.tcp(), None);
        assert_eq!(enr.tcp6(), None);
        dbg!("here1");
        assert_eq!(enr.signature(), &signature[..]);
        dbg!("here2");
        assert_eq!(pubkey, expected_pubkey);
        dbg!("here3");
        assert_eq!(enr.node_id().raw().to_vec(), expected_node_id);

        assert!(enr.verify());
    }

    #[cfg(feature = "libsecp256k1")]
    #[test]
    fn test_read_enr() {
        let text = "-Iu4QM-YJF2RRpMcZkFiWzMf2kRd1A5F1GIekPa4Sfi_v0DCLTDBfOMTMMWJhhawr1YLUPb5008CpnBKrgjY3sstjfgCgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQP8u1uyQFyJYuQUTyA1raXKhSw1HhhxNUQ2VE52LNHWMIN0Y3CCIyiDdWRwgiMo";
        let enr = text.parse::<DefaultEnr>().unwrap();
        dbg!(enr.ip());
        dbg!(enr.udp());
        dbg!(enr.tcp());
    }

    #[cfg(feature = "libsecp256k1")]
    #[test]
    fn test_encode_test_vector_2() {
        let key = secp256k1::SecretKey::parse_slice(
            &hex::decode("b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291")
                .unwrap(),
        )
        .unwrap();

        let signature = hex::decode("7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c").unwrap();

        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let udp = 30303;

        let enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.ip(ip.into());
            builder.udp(udp);
            builder.build(&key).unwrap()
        };

        assert_eq!(enr.signature(), &signature[..]);
    }

    #[cfg(feature = "libsecp256k1")]
    #[test]
    fn test_encode_decode_secp256k1() {
        let mut rng = rand::thread_rng();
        let key = secp256k1::SecretKey::random(&mut rng);
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let tcp = 3000;

        let enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.ip(ip.into());
            builder.tcp(tcp);
            builder.build(&key).unwrap()
        };

        let encoded_enr = rlp::encode(&enr);

        let decoded_enr = rlp::decode::<DefaultEnr>(&encoded_enr).unwrap();

        assert_eq!(decoded_enr.id(), Some("v4".into()));
        assert_eq!(decoded_enr.ip(), Some(ip));
        assert_eq!(decoded_enr.tcp(), Some(tcp));
        // Must compare encoding as the public key itself can be different
        assert_eq!(decoded_enr.public_key().encode(), key.public().encode());
        assert!(decoded_enr.verify());
    }

    #[cfg(feature = "rust-secp256k1")]
    #[test]
    fn test_encode_decode_c_secp256k1() {
        let mut rng = c_secp256k1::rand::thread_rng();
        let key = c_secp256k1::SecretKey::new(&mut rng);
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let tcp = 3000;

        let enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.ip(ip.into());
            builder.tcp(tcp);
            builder.build(&key).unwrap()
        };

        let encoded_enr = rlp::encode(&enr);

        let decoded_enr = rlp::decode::<Enr<c_secp256k1::SecretKey>>(&encoded_enr).unwrap();

        assert_eq!(decoded_enr.id(), Some("v4".into()));
        assert_eq!(decoded_enr.ip(), Some(ip));
        assert_eq!(decoded_enr.tcp(), Some(tcp));
        // Must compare encoding as the public key itself can be different
        assert_eq!(decoded_enr.public_key().encode(), key.public().encode());
        assert!(decoded_enr.verify());
    }

    #[cfg(feature = "ed25519")]
    #[test]
    fn test_encode_decode_ed25519() {
        let mut rng = rand::thread_rng();
        let key = ed25519_dalek::Keypair::generate(&mut rng);
        let ip = Ipv4Addr::new(10, 0, 0, 1);
        let tcp = 30303;

        let enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.ip(ip.into());
            builder.tcp(tcp);
            builder.build(&key).unwrap()
        };

        let encoded_enr = rlp::encode(&enr);
        let decoded_enr = rlp::decode::<Enr<CombinedKey>>(&encoded_enr).unwrap();

        assert_eq!(decoded_enr.id(), Some("v4".into()));
        assert_eq!(decoded_enr.ip(), Some(ip));
        assert_eq!(decoded_enr.tcp(), Some(tcp));
        assert_eq!(decoded_enr.public_key().encode(), key.public().encode());
        assert!(decoded_enr.verify());
    }

    #[test]
    fn test_add_key() {
        let mut rng = rand::thread_rng();
        let key = secp256k1::SecretKey::random(&mut rng);
        let ip = Ipv4Addr::new(10, 0, 0, 1);
        let tcp = 30303;

        let mut enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.ip(ip.into());
            builder.tcp(tcp);
            builder.build(&key).unwrap()
        };

        assert!(enr.insert("random", Vec::new(), &key).is_ok());
        assert!(enr.verify());
    }

    #[test]
    fn test_set_ip() {
        let mut rng = rand::thread_rng();
        let key = secp256k1::SecretKey::random(&mut rng);
        let tcp = 30303;
        let ip = Ipv4Addr::new(10, 0, 0, 1);

        let mut enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.tcp(tcp);
            builder.build(&key).unwrap()
        };

        assert!(enr.set_ip(ip.into(), &key).is_ok());
        assert_eq!(enr.id(), Some("v4".into()));
        assert_eq!(enr.ip(), Some(ip));
        assert_eq!(enr.tcp(), Some(tcp));
        assert!(enr.verify());

        // Compare the encoding as the key itself can be differnet
        assert_eq!(enr.public_key().encode(), key.public().encode(),);
    }

    #[test]
    fn ip_mutation_static_node_id() {
        let mut rng = rand::thread_rng();
        let key = secp256k1::SecretKey::random(&mut rng);
        let tcp = 30303;
        let udp = 30304;
        let ip = Ipv4Addr::new(10, 0, 0, 1);

        let mut enr = {
            let mut builder = EnrBuilder::new("v4");
            builder.ip(ip.into());
            builder.tcp(tcp);
            builder.udp(udp);
            builder.build(&key).unwrap()
        };

        let node_id = enr.node_id();

        enr.set_udp_socket("192.168.0.1:800".parse::<SocketAddr>().unwrap(), &key)
            .unwrap();
        assert_eq!(node_id, enr.node_id());
        assert_eq!(
            enr.udp_socket(),
            "192.168.0.1:800".parse::<SocketAddr>().unwrap().into()
        );
    }

    #[cfg(feature = "ed25519")]
    #[test]
    fn combined_key_can_decode_all() {
        // generate a random secp256k1 key
        let mut rng = rand::thread_rng();
        let key = secp256k1::SecretKey::random(&mut rng);
        let ip = Ipv4Addr::new(192, 168, 0, 1);
        let enr_secp256k1 = EnrBuilder::new("v4")
            .ip(ip.into())
            .tcp(8000)
            .build(&key)
            .unwrap();

        // encode to base64
        let base64_string_secp256k1 = enr_secp256k1.to_base64();

        // generate a random ed25519 key
        let key = ed25519_dalek::Keypair::generate(&mut rng);
        let enr_ed25519 = EnrBuilder::new("v4")
            .ip(ip.into())
            .tcp(8000)
            .build(&key)
            .unwrap();

        // encode to base64
        let base64_string_ed25519 = enr_ed25519.to_base64();

        // decode base64 strings of varying key types
        // decode the secp256k1 with default Enr
        let _decoded_enr_secp256k1: DefaultEnr = base64_string_secp256k1.parse().unwrap();
        // decode ed25519 ENRs
        let _decoded_enr_ed25519: Enr<ed25519_dalek::Keypair> =
            base64_string_ed25519.parse().unwrap();

        // use the combined key to be able to decode either
        let _decoded_enr: Enr<CombinedKey> = base64_string_secp256k1
            .parse()
            .expect("Can decode both secp");
        let _decoded_enr: Enr<CombinedKey> = base64_string_ed25519.parse().unwrap();
    }
}
