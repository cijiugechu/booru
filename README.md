![ci-badge][] [![crates.io version]][crates.io link] 
# `booru`
An async Booru client for Rust.

> Note: This project has been forked from [booru-rs](https://github.com/ajiiisai/booru-rs) since September of 2023, but a lot has changed.

##  Overview
The client currently supports:
- [x] Gelbooru
- [x] Safebooru
- [x] Danbooru
- [x] Konachan
- [x] Rule 34

## Example
Remember to bring the `Client` trait into scope with `use booru::client::Client;`.
```rust
use booru::{
        client::{gelbooru::GelbooruClient, generic::*, Client},
        model::gelbooru::GelbooruRating,
    };

let posts = GelbooruClient::builder()
    .tag("kafuu_chino")
    .tag("2girls")
    .rating(GelbooruRating::General)
    .sort(GelbooruSort::Score)
    .limit(5)
    .random(true)
    .blacklist_tag(GelbooruRating::Explicit)
    .build()
    .get()
    .await
    .expect("There was an error retrieving posts from the API");

// If you want to customize http client, you can use `builder_with_http_client`:
let http_client_builder = reqwest::ClientBuilder::new()
                          .proxy(reqwest::Proxy::http("http://127.0.0.1:7890").unwrap());

let posts = GelbooruClient::builder_with_http_client(http_client_builder)
    .tag("kafuu_chino")
    .tag("2girls")
    .rating(GelbooruRating::General)
    .sort(GelbooruSort::Score)
    .limit(5)
    .random(true)
    .blacklist_tag(GelbooruRating::Explicit)
    .build()
    .get()
    .await
    .expect("There was an error retrieving posts from the API");
```

[ci-badge]: https://img.shields.io/github/actions/workflow/status/cijiugechu/booru/ci.yml?branch=main
[crates.io link]: https://crates.io/crates/booru
[crates.io version]: https://img.shields.io/crates/v/booru.svg?style=flat-square
