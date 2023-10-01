use async_trait::async_trait;
use itoa::Buffer;

use super::{Client, ClientBuilder};
use crate::model::gelbooru::*;

/// Client that sends requests to the Gelbooru API to retrieve the data.
pub struct GelbooruClient(ClientBuilder<Self>);

impl From<ClientBuilder<Self>> for GelbooruClient {
    fn from(value: ClientBuilder<Self>) -> Self {
        Self(value)
    }
}

#[async_trait]
impl Client for GelbooruClient {
    type Post = GelbooruPost;

    const URL: &'static str = "https://gelbooru.com";
    const SORT: &'static str = "sort:";

    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<GelbooruPost, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("id", Buffer::new().format(id)),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await?;

        Ok(response.posts[0].clone())
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let tag_string = builder.tags.join(" ");
        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tag_string),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await?;

        Ok(response.posts)
    }

    /// retrieve the most top rated posts
    async fn get_popular(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", "sort:score:desc"),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await?;

        Ok(response.posts)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get_by_page(&self, page: u32) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let tag_string = builder.tags.join(" ");
        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tag_string),
                ("pid", Buffer::new().format(builder.limit * page)),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await?;

        Ok(response.posts)
    }
}
