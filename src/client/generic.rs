use crate::model::{
    danbooru::DanbooruRating, gelbooru::GelbooruRating, rule34::Rule34Rating,
    safebooru::SafebooruRating,
};
use serde::{Deserialize, Serialize};
use std::fmt;

pub enum Rating {
    Danbooru(DanbooruRating),
    Gelbooru(GelbooruRating),
    Safebooru(SafebooruRating),
    Rule34(Rule34Rating),
}

impl From<DanbooruRating> for Rating {
    fn from(value: DanbooruRating) -> Self {
        Rating::Danbooru(value)
    }
}

impl From<GelbooruRating> for Rating {
    fn from(value: GelbooruRating) -> Self {
        Rating::Gelbooru(value)
    }
}

impl From<SafebooruRating> for Rating {
    fn from(value: SafebooruRating) -> Self {
        Rating::Safebooru(value)
    }
}

impl From<Rule34Rating> for Rating {
    fn from(value: Rule34Rating) -> Self {
        Rating::Rule34(value)
    }
}

#[derive(Debug, Clone)]
pub enum Sort {
    Id,
    Score,
    Rating,
    User,
    Height,
    Width,
    Source,
    Updated,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCompleteItem {
    pub value: String,
    pub label: String,
}
