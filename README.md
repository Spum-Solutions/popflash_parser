# popflash_parser

`popflash_parser` is a crate to parse and translate match information from <https://popflash.site/> This tool is currently working as of February 27th 2022, although other similar tools have been borked when `popflash` has changed it's web page structure so take caution when using.

Methods are used to get the match data from a url or match ID, they are functionally equivalent to the end user, only differing in what is used to call the function (a url, or match ID)

``` rust
## use popflash_parser::*;
###[tokio::test]
## async fn test() {
assert_eq!(
    Match::from_url("https://popflash.site/match/1281644").await.unwrap(),
    Match::from_id("1281644").await.unwrap()
);
## }
```rust

A `Match` struct (see example output here: <https://pastebin.com/AVt1zVV8>) can be printed as follows

```
use popflash_parser::*;
## async fn test() {
let match_data  = Match::from_id(1281644).await.unwrap();
println!("Match::Match output");
println!("{:#?}", match_data);

// Convert to JSON string
let json_string = serde_json::to_string(&match_data).unwrap();
println!("String output");
println!("{}", json_string);

// Convert  to JSON object
let json_object = serde_json::Value::from(json_string);
println!("serde_json::Value output");
println!("{}", json_object);
## }
```rust
