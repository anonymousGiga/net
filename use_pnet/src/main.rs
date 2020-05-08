use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use std::env;


fn main() {
    let interface_name = env::args().nth(1).unwrap();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("Error get interface");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Other"),
        Err(e) => panic!("error: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            },
            Err(e) => {
                println!("Some error: {}", e);
            }
        }
    }
}

fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            println!("Tcp packet {}: {} to {}: {}",
                            header.get_source(),
                            tcp.get_source(),
                            header.get_destination(),
                            tcp.get_destination());
                        }
                    }
                    _ => println!("igno"),
                }
            }
        }
        _ => println!("ignoring"),
    }
}