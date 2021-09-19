use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {
        println!("a client connected");
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let byte_read = stream.read(&mut buf)?;
        if byte_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..byte_read])?;
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
