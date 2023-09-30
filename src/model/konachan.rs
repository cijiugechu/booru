//! Models for Konachan
use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KonachanPost {
    pub id: u32,
    pub created_at: u64,
    pub creator_id: u32,
    pub tags: String,
    pub change: u32,
    pub rating: Option<KonachanRating>,
    pub parent_id: Option<u32>,
    pub source: String,
    pub md5: Option<String>,
    pub file_url: Option<String>,
    pub file_size: u32,
    pub preview_url: Option<String>,
    pub preview_width: u32,
    pub preview_height: u32,
    pub actual_preview_width: u32,
    pub actual_preview_height: u32,
    pub sample_url: Option<String>,
    pub sample_width: u32,
    pub sample_height: u32,
    pub sample_file_size: u32,
    pub jpeg_url: Option<String>,
    pub jpeg_width: u32,
    pub jpeg_height: u32,
    pub jpeg_file_size: u32,
    pub score: i32,
    pub has_children: bool,
    pub is_shown_in_index: bool,
    pub is_held: bool,
}

/// Post's rating. Check the [Konachan's ratings help](https://konachan.com/help/ratings)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum KonachanRating {
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "s")]
    Safe,
    #[serde(rename = "q")]
    Questionable,
}

impl From<KonachanRating> for String {
    fn from(rating: KonachanRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for KonachanRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
