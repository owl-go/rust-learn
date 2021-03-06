use std::str;
use std::{io, net::UdpSocket};
fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    socket.connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;
        let mut buf = [0u8; 1024];
        socket.recv_from(&mut buf)?;
        println!(
            "recieve from server:{}",
            str::from_utf8(&buf).expect("could not write byte with string")
        );
    }
}
