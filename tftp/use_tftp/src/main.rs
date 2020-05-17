use tftp_server::server::TftpServerBuilder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//use std::str::FromStr;

fn main() {
    // let addr = format!("0.0.0.0.{}", 69);
    // let socket_addr = SocketAddr::from_str(addr.as_str()).unwrap();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 69);
    let builder = TftpServerBuilder::new().addr(socket_addr);
    let mut server = builder.build().unwrap();
    server.run().expect("some error");
}
