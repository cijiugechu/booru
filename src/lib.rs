//! ### Usage
//! ```
//! use booru::{danbooru::{DanbooruClient, DanbooruRating}, Sort, Client};
//!
//! #[tokio::main]
//! async fn main() {
//!     let posts = DanbooruClient::builder()
//!         .rating(DanbooruRating::General)
//!         .sort(Sort::Score)
//!         .build()
//!         .get()
//!         .await
//!         .expect("There was an error. (•-•)");
//!
//!     match posts.first() {
//!         Some(post) => println!("{:?}", post),
//!         None => panic!("Well... \"No posts found?\""),
//!     }
//! }
//! ```

pub mod client;
pub mod model;

// Conveience
pub use client::{generic::Sort, Client};

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

pub mod konachan {
    pub use crate::client::konachan::*;
    pub use crate::model::konachan::*;
}

pub mod rule34 {
    pub use crate::client::rule34::*;
    pub use crate::model::rule34::*;
}
