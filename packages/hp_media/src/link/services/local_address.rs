use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub(crate) fn local_address(server: IpAddr) -> IpAddr {
    match server {
        IpAddr::V4(address) if address.is_loopback() => Ipv4Addr::LOCALHOST.into(),
        IpAddr::V4(_) => Ipv4Addr::UNSPECIFIED.into(),
        IpAddr::V6(address) if address.is_loopback() => Ipv6Addr::LOCALHOST.into(),
        IpAddr::V6(_) => Ipv6Addr::UNSPECIFIED.into(),
    }
}
