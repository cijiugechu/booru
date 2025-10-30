[![Cargo](https://img.shields.io/crates/v/booru.svg)](https://crates.io/crates/booru) [![Documentation](https://docs.rs/booru/badge.svg)](https://docs.rs/booru)
# `booru`
An async Booru client for Rust.

> Note: This project has been forked from [booru-rs](https://github.com/ajiiisai/booru-rs) since September of 2023, but a lot has changed.

##  Overview
The client currently supports:
- [x] Gelbooru
- [x] Safebooru
- [x] Danbooru
- [x] Rule 34

## Example

Remember to bring the `Client` trait into scope with `use booru::client::Client;`.
```rust
use booru::{
        client::{gelbooru::GelbooruClient, generic::*, Client},
        model::gelbooru::GelbooruRating,
    };

#[tokio::main]
async fn main() {
    let posts = GelbooruClient::builder()
        .tag("kafuu_chino")
        .tag("2girls")
        .rating(GelbooruRating::General)
        .sort(Sort::Score)
        .limit(5)
        .random()
        .blacklist_tag(GelbooruRating::Explicit)
        .build()
        .get()
        .await
        .expect("There was an error retrieving posts from the API");
}
```

### Customizing http client

If you want to customize http client, you can use `builder_with_http_client`:
```rust
use booru::{
        client::{gelbooru::GelbooruClient, generic::*, Client},
        model::gelbooru::GelbooruRating,
    };
use reqwest;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let http_client_builder = reqwest::ClientBuilder::new()
                            .timeout(Duration::from_secs(10));

    let posts = GelbooruClient::builder_with_http_client(http_client_builder)
        .tag("kafuu_chino")
        .limit(5)
        .build()
        .get()
        .await
        .expect("There was an error retrieving posts from the API");
}
```

## Tests

> [!WARNING]
> To run the `gelbooru` tests, set the `GELBOORU_API_KEY` and
> `GELBOORU_USER_ID` env variables, then run with `cargo test -- --nocapture
> --ignored`.

```bash
# Run all tests except gelbooru
cargo test -- --nocapture

# Run all tests
export GELBOORU_API_KEY="your_api_key"
export GELBOORU_USER_ID="your_user_id"
cargo test -- --nocapture --ignored
