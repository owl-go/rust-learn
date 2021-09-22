use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}
fn main() {
    let config = ServerConfig {
        workers: 100,
        ignore: false,
        auth_server: Some("auth.string.io".to_string()),
    };
    {
        println!("json:");
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serilalized:{}", serialized);
        let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
        println!("deserialized:{:#?}", deserialized);
    }

    {
        println!("yaml:");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("serilalized:{}", serialized);
        let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        println!("deserialized:{:#?}", deserialized);
    }
}
