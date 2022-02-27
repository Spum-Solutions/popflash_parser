# popflash_parser
A rust library / tool to parse match results from `https://popflash.site`. Example output seen here https://pastebin.com/MKUyjx9b.

## Getting Started
Add the library as a dependency in your `cargo.toml` such as follows

```
`Cargo.toml`

[dependencies]
popflash_parser = "0.1.0"
serde_json = "~1.0"

`src/main.rs`

fn main() {
    let match_data = popflash_parser::synchronous::match_from_id("1281644");
    let json_string = serde_json::to_string(&game).unwrap();
    let json_json = serde_json::Value::from(json_string);
}
```

An asynchronous interface is also available (and is the default) but requires an asynchronous context (such as tokio) and aims to keep the same interface

```
`Cargo.toml`
[dependencies]
popflash_parser = "0.1.0"
serde_json = "~1.0"
tokio = { version = "1", features = ["full"] }

`src/main.rs`

#[tokio::main]
async fn main() {
    let match_data = popflash_parser::match_from_id("1281644").await;
    let json_string = serde_json::to_string(&game).unwrap();
    let json_json = serde_json::Value::from(json_string);
}
```

## todos

## 
