//! [`PortableHash`] implementations for standard library types.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use crate::{PortableHash, PortableHasher};

impl PortableHash for IpAddr {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            IpAddr::V4(addr) => {
                state.write_u8(0);
                addr.portable_hash(state);
            }
            IpAddr::V6(addr) => {
                state.write_u8(1);
                addr.portable_hash(state);
            }
        }
    }
}

impl PortableHash for Ipv4Addr {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write(&self.octets());
    }
}

impl PortableHash for Ipv6Addr {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write(&self.octets());
    }
}

impl PortableHash for SocketAddr {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            SocketAddr::V4(addr) => {
                state.write_u8(0);
                addr.portable_hash(state);
            }
            SocketAddr::V6(addr) => {
                state.write_u8(1);
                addr.portable_hash(state);
            }
        }
    }
}

impl PortableHash for SocketAddrV4 {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.ip().portable_hash(state);
        state.write_u16(self.port());
    }
}

impl PortableHash for SocketAddrV6 {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.ip().portable_hash(state);
        state.write_u16(self.port());
        state.write_u32(self.flowinfo());
        state.write_u32(self.scope_id());
    }
}
