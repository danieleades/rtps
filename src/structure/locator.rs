use std::net::{SocketAddrV4, SocketAddrV6};

/// Generalisation of a possible connection
#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Locator {
    // Invalid,
    // Reserved,
    /// An IPv4 UDP socket address
    Udpv4(SocketAddrV4),

    /// An IPv4 UDP socket address
    Udpv6(SocketAddrV6),
    /* AddressInvalid,
     * PortInvalid, */
}

impl From<SocketAddrV4> for Locator {
    fn from(socket_addr: SocketAddrV4) -> Self {
        Self::Udpv4(socket_addr)
    }
}
