use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use portable_hash::BuildPortableHasher;
use crate::FixtureDB;

pub fn test_std(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    // Ipv4Addr — hashes raw octets
    fixtures.test_fixture("ipv4_localhost", Ipv4Addr::new(127, 0, 0, 1));
    fixtures.test_fixture("ipv4_unspecified", Ipv4Addr::UNSPECIFIED);
    fixtures.test_fixture("ipv4_broadcast", Ipv4Addr::BROADCAST);
    fixtures.test_fixture("ipv4_custom", Ipv4Addr::new(192, 168, 1, 100));

    // Ipv6Addr — hashes 16 octets
    fixtures.test_fixture("ipv6_localhost", Ipv6Addr::LOCALHOST);
    fixtures.test_fixture("ipv6_unspecified", Ipv6Addr::UNSPECIFIED);
    fixtures.test_fixture("ipv6_custom", Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1));

    // IpAddr — enum discriminant (0=V4, 1=V6) + inner address
    fixtures.test_fixture("ipaddr_v4_localhost", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    fixtures.test_fixture("ipaddr_v6_localhost", IpAddr::V6(Ipv6Addr::LOCALHOST));

    // SocketAddrV4 — ip octets + u16 port
    fixtures.test_fixture("socketaddrv4_localhost_80", SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80));
    fixtures.test_fixture("socketaddrv4_localhost_443", SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 443));
    fixtures.test_fixture("socketaddrv4_custom", SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 8080));

    // SocketAddrV6 — ip + port + flowinfo + scope_id
    fixtures.test_fixture("socketaddrv6_localhost_80", SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0));
    fixtures.test_fixture("socketaddrv6_with_flowinfo", SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 42, 0));
    fixtures.test_fixture("socketaddrv6_with_scope", SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 7));

    // SocketAddr — enum discriminant (0=V4, 1=V6) + inner address
    fixtures.test_fixture("socketaddr_v4", SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80)));
    fixtures.test_fixture("socketaddr_v6", SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0)));
}
