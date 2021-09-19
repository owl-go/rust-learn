use std::{io, net::UdpSocket};
fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("bind port 8080 failed");
    loop {
        let mut buf = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
}
