extern crate redis;
// use redis::cluster::ClusterClient;
// use redis::Commands;
use redis::AsyncCommands;
#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // let nodes = vec!["redis://127.0.0.1/"];
    // let client = ClusterClient::open(nodes).unwrap();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let mut con = client.get_async_connection().await?;
    let _: () = con.set("rust_key", "rust-test").await?;
    let rv: String = con.get("rust_key").await?;
    println!("value is :{}", rv);
    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut con)
        .await?;
    // let result = redis::cmd("SET")
    //     .arg(&["key2", "bar"])
    //     .query_async(&mut con)
    //     .await;
    // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

    Ok(())
}
