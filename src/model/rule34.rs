//! Models for Rule34
use core::fmt;

use serde::{Deserialize, Serialize};

/// Individual post
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule34Post {
    /// The ID of the post
    pub id: u32,
    /// Post's score
    pub score: u32,
    /// Post's image width
    pub width: u32,
    /// Post's image height
    pub height: u32,
    /// Post's preview url
    pub preview_url: Option<String>,
    /// Post's sample url
    pub sample_url: Option<String>,
    /// Post's image file url
    pub file_url: String,
    /// Post's directory
    pub directory: u32,
    /// Post's tags
    pub tags: String,
    /// Post's image name (with extension)
    pub image: String,
    /// Post's image source
    pub source: String,
    /// Post's rating
    pub rating: Rule34Rating,
    /// Post's has_notes
    pub has_notes: bool,
    pub sample: bool,
    pub sample_width: Option<u32>,
    pub sample_height: Option<u32>,
    pub parent_id: u32,
    pub owner: String,
    pub comment_count: u32,
    pub change: u32,
    pub hash: String,
}

/// Post's rating. Check the [Rule34's ratings wiki](https://rule34.xxx/index.php?page=help&topic=rating)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Rule34Rating {
    Explicit,
    Questionable,
    Safe,
}

impl From<Rule34Rating> for String {
    fn from(rating: Rule34Rating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for Rule34Rating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lovercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lovercase_tag}")
    }
}
