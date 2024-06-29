use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{collections::HashMap, fs::File};

#[derive(Serialize, Deserialize)]
struct Config {
    hashmap: HashMap<String, String>,
}

fn main() {
    // 创建一个示例的 HashMap
    let mut hashmap = HashMap::new();
    hashmap.insert("key1".to_string(), "value1".to_string());
    hashmap.insert("key2".to_string(), "value2".to_string());

    // 将 HashMap 存储到 Config 结构体中
    //let config = Config { hashmap };

    // 序列化 Config 结构体为 TOML 格式
    let serialized = toml::to_string(&hashmap).unwrap();

    // 写入到文件
    let mut file = File::create("config.conf").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}
