use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// #![allow(dead_code, unused_variables)]
const LOCAL_SERVER: &str = "127.0.0.1:8888";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(LOCAL_SERVER).await?;
    loop {
        let (mut sock, add) = listener.accept().await?;
        println!("{} connected", add);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match sock.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        // print!("recv msg:{}", std::str::from_utf8(&buf).unwrap());
                        n
                    }
                    Err(e) => {
                        eprintln!("failed to read from sock;err={:?}", e);
                        return;
                    }
                };
                if let Err(e) = sock.write(&buf[..n]).await {
                    eprintln!("failed to write socket;err={:?}", e);
                    return;
                }
            }
        });
    }
}
