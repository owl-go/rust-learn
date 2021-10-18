use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    // let (tx, mut rx) = broadcast::channel(10);
    let (tx, mut _rx) = broadcast::channel(16);
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("{} connected!", addr);
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (r, mut w) = socket.split();
            let mut reader = BufReader::new(r);
            let mut msg = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut msg) => {
                        if result.unwrap()==0{
                            break;
                        }
                        println!("{}",msg);
                        tx.send((msg.clone(),addr)).unwrap();
                        msg.clear();
                    }
                    result = rx.recv() => {
                        let (msg,other) = result.unwrap();
                        if other != addr {
                            w.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
