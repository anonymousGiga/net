use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

fn main() {
    let mut v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);

    assert_eq!(v4.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    v4.set_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    assert_eq!(v4.ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));

    v6.set_ip(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 1, 1)));

    assert_eq!(v4.port(), 8080);
    v4.set_port(1025);

    v6.set_port(1025);

    assert_eq!(v4.is_ipv4(), true);
    assert_eq!(v6.is_ipv4(), false);

    assert_eq!(v4.is_ipv6(), false);
    assert_eq!(v6.is_ipv6(), true);
}
