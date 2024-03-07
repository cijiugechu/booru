#![doc = include_str!("../README.md")]

//! ### Get by page
//!
//! ```
//! use booru::{danbooru::DanbooruClient, Client};
//!
//! #[tokio::main]
//! async fn main() {
//!     let posts = DanbooruClient::builder()
//!         .tag("kafuu_chino")
//!         .limit(5)
//!         .build()
//!         .get_by_page(2)
//!         .await
//!         .expect("There was an error retrieving posts from the API");
//!
//!     println!("{:?}", posts);
//! }
//! ```

//! ### Get by popular
//!
//! ```
//! use booru::{danbooru::DanbooruClient, Client};
//!
//! #[tokio::main]
//! async fn main() {
//!     let posts = DanbooruClient::builder()
//!         .limit(5)
//!         .build()
//!         .get_popular()
//!         .await
//!         .expect("There was an error retrieving posts from the API");
//!
//!     println!("{:?}", posts);
//! }
//! ```

//! ### Get Autocomplete
//!
//! ```rust
#![doc = include_str!("../examples/get_autocomplete.rs")]
//! ```

pub mod client;
pub mod model;

// Conveience
pub use client::{
    generic::{AutoCompleteItem, Sort},
    Client,
};

pub mod safebooru {
    pub use crate::client::safebooru::*;
    pub use crate::model::safebooru::*;
}

pub mod gelbooru {
    pub use crate::client::gelbooru::*;
    pub use crate::model::gelbooru::*;
}

pub mod danbooru {
    pub use crate::client::danbooru::*;
    pub use crate::model::danbooru::*;
}

pub mod rule34 {
    pub use crate::client::rule34::*;
    pub use crate::model::rule34::*;
}
