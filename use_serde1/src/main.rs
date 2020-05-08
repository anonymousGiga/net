// serde crate 是 Serde 生态的核心。
// serde_derive crate 提供必要的工具，
// 使用过程宏来派生 Serialize 和 Deserialize。
// 但是serde只提供序列化和反序列化的框架，具体的操作还需要依赖具体的包，
// 如serde_json和serde_yaml等。
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

fn main() {
    let config = ServerConfig {
        workers: 100,
        ignore: false,
        auth_server: Some("auth.server.io".to_string()),
    };

    {
        println!("json:");
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serialized: {}", serialized);

        let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
        println!("deserialized: {:#?}", deserialized);
    }

    {
        println!("yaml:");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("serialized: {}", serialized);

        let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        println!("deserialized: {:#?}", deserialized);
    }

    println!("Hello, world!");
}
