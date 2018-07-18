#![deny(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

//! # A rust port of bitcoin network
//!
//!   1. Data structures
//!   2. Interfaces
//!

use std::net::IpAddr;

/// The type that we do not know the meaning yet.
pub struct Unknown<T> {
    inner: T,
}

/// Network node
pub struct Node {}
/// Network address
pub struct Address {
    inner: Service,
}
/// Network Service: A combination of a network address (CNetAddr) and a (TCP) port
pub struct Service {
    inner: NetAddr,
}
/// Services flags
pub enum ServiceFlags {
    /// Nothing
    NODE_NONE = 0,
    /// NODE_NETWORK means that the node is capable of serving the complete block chain. It is currently
    /// set by all Bitcoin Core non pruned nodes, and is unset by SPV clients or other light clients.
    NODE_NETWORK = (1 << 0),
    /// NODE_GETUTXO means the node is capable of responding to the getutxo protocol request.
    /// Bitcoin Core does not support this but a patch set called Bitcoin XT does.
    /// See BIP 64 for details on how this is implemented.
    NODE_GETUTXO = (1 << 1),
    /// NODE_BLOOM means the node is capable and willing to handle bloom-filtered connections.
    /// Bitcoin Core nodes used to support this by default, without advertising this bit,
    /// but no longer do as of protocol version 70011 (= NO_BLOOM_VERSION)
    NODE_BLOOM = (1 << 2),
    /// NODE_WITNESS indicates that a node can be asked for blocks and transactions including
    /// witness data.
    NODE_WITNESS = (1 << 3),
    /// NODE_XTHIN means the node supports Xtreme Thinblocks
    /// If this is turned off then the node will not service nor make xthin requests
    NODE_XTHIN = (1 << 4),
    /// NODE_NETWORK_LIMITED means the same as NODE_NETWORK with the limitation of only
    /// serving the last 288 (2 day) blocks
    /// See BIP159 for details on how this is implemented.
    NODE_NETWORK_LIMITED = (1 << 10),

    // Bits 24-31 are reserved for temporary experiments. Just pick a bit that
    // isn't getting used, or one not being used much, and notify the
    // bitcoin-development mailing list. Remember that service bits are just
    // unauthenticated advertisements, so your code must be robust against
    // collisions and other cases where nodes may be advertising a service they
    // do not actually support. Other service bits should be allocated via the
    // BIP process.
}
/// Ban Reason
pub enum BanReason {
    /// Unknown
    BanReasonUnknown          = 0,
    /// Node Misbehaving
    BanReasonNodeMisbehaving  = 1,
    /// Manually Added
    BanReasonManuallyAdded    = 2
}
/// IP address (IPv6, or IPv4 using mapped IPv6 range (::FFFF:0:0/96))
pub struct NetAddr {
    ip: IpAddr,
    // for scoped/link-local ipv6 addresses
    scope_id: Unknown<u32>,
}
/// Sub network
pub struct SubNet {}
/// Serialized network message
pub struct RawNetMessage {}
/// Network Service implementation
pub struct NetworkServiceImpl {}
/// The address to be banned
pub enum BanAddr {
    /// Ban a NetAddr
    Net(NetAddr),
    /// Ban a sub network
    Sub(SubNet),
}


/// Handle bitcoin message relay
pub trait RelayService {}
/// Handle bitcoin message sync
pub trait SyncService {}
/// All Netowkr Service private interface
trait NetworkServicePrivate {}

/// All Network Service
pub trait NetworkService {
    /// Interrupt the network sockets
    fn interrupt() {}
    /// If the network is active
    fn network_active() -> bool { true }
    /// Set if the network is active
    fn set_network_active(active: bool) {}
    /// Open a network connection
    fn open_network_connection(
        addr: Address,
        count_failure: bool,
        grant_outbound: Unknown<()>,
        str_dest: Unknown<&str>,
        one_shot: bool,
        feeler: bool,
        manual_connection: bool,
    ) {}
    /// Check the ing coming nonce
    fn check_incoming_nonce(nonce: u64) -> bool { true }
    /// TODO: push message to message queue
    fn push_message(node: &Node, msg: RawNetMessage) {}


    /// Get address count
    fn get_address_count() -> usize { 0 }
    /// Set services
    fn set_services(addr: &Service, services: ServiceFlags) {}
    /// Mark address as good
    fn mark_address_good(addr: &Address) {}
    /// Add new addresses
    fn add_new_addresses(
        addrs: &[Address],
        addr_from: &Address,
        time_penalty: Unknown<i64>,
    ) {}
    /// Get addresses
    fn get_addresses() -> Vec<Address> { Vec::new() }


    /// Ban a address
    fn ban(
        addr: &BanAddr,
        reason: BanReason,
        ban_time_offset: i64,
        since_unix_epoch: bool,
    ) {}
}

impl NetworkServicePrivate for NetworkServiceImpl {}
impl NetworkService for NetworkServiceImpl {}
