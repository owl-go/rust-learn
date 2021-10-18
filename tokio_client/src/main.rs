use std::io;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};
const LOCAL_SERVER: &str = "127.0.0.1:8888";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TcpStream::connect(LOCAL_SERVER).await?;
    let (tx, mut rx) = mpsc::channel::<String>(3);
    tokio::spawn(async move {
        let (r, mut w) = client.split();
        let mut r = BufReader::new(r);
        let mut line = String::new();
        loop {
            if let Some(str) = rx.recv().await {
                w.write_all(str.as_bytes()).await.unwrap();
            } else {
                continue;
            }
            let read_byte = r.read_line(&mut line).await.unwrap();
            if read_byte > 0 {
                println!("recv msg from server:{}", line);
            }
            line.clear();
        }
    });
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("reading from stdin failed");
        tx.send(buf.clone()).await.unwrap();
        buf.clear();
    }
}
