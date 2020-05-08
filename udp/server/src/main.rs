use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("size = {}", amt);

        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, src)?;
    }
}
