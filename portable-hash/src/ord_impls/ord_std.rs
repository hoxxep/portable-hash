use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    time::{Instant, SystemTime},
};
use crate::PortableOrd;

// TODO(stabilisation): do we trust enum ordering is stable for IpAddr?
impl PortableOrd for IpAddr {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for Ipv4Addr {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for Ipv6Addr {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

// TODO(stabilisation): do we trust enum ordering is stable for SocketAddr?
impl PortableOrd for SocketAddr {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for SocketAddrV4 {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for SocketAddrV6 {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for Instant {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}

impl PortableOrd for SystemTime {
    const CAN_USE_UNSTABLE_SORT: bool = true;
    const I_KNOW_WHAT_I_AM_DOING: () = ();
}
