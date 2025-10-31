use super::{Client, ClientBuilder, generic::AutoCompleteItem};
use crate::model::danbooru::*;

use itoa::Buffer;
use reqwest::{header, header::HeaderMap};

// This is only here because of Danbooru, thanks Danbooru, really cool :)
fn get_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct DanbooruClient(ClientBuilder<Self>);

impl From<ClientBuilder<Self>> for DanbooruClient {
    fn from(value: ClientBuilder<Self>) -> Self {
        Self(value)
    }
}

impl Client for DanbooruClient {
    type Post = DanbooruPost;

    const URL: &'static str = "https://danbooru.donmai.us";
    const SORT: &'static str = "order:";

    /// Directly get a post by its unique Id
    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts/{id}.json"))
            .headers(get_headers())
            .send()
            .await?
            .json::<DanbooruPost>()
            .await?;
        Ok(response)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    #[tracing::instrument(skip(self))]
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
            .query(&[
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<DanbooruPost>>()
            .await?;

        Ok(response)
    }

    /// retrieve the most top rated posts
    #[tracing::instrument(skip(self))]
    async fn get_popular(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/explore/posts/popular.json"))
            .headers(get_headers())
            .query(&[("limit", Buffer::new().format(builder.limit))])
            .send()
            .await?
            .json::<Vec<DanbooruPost>>()
            .await?;

        Ok(response)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    #[tracing::instrument(skip(self))]
    async fn get_by_page(&self, page: u32) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
            .query(&[
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tag_string),
                ("page", Buffer::new().format(page)),
            ])
            .send()
            .await?
            .json::<Vec<DanbooruPost>>()
            .await?;

        Ok(response)
    }

    #[tracing::instrument(skip(self))]
    async fn get_autocomplete<Input: Into<String> + Send + std::fmt::Debug>(
        &self,
        input: Input,
    ) -> Result<Vec<AutoCompleteItem>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/autocomplete.json"))
            .headers(get_headers())
            .query(&[
                ("limit", Buffer::new().format(builder.limit)),
                ("search[type]", "tag_query"),
                ("search[query]", &input.into()),
                ("version", "1"),
            ])
            .send()
            .await?
            .json::<Vec<AutoCompleteItem>>()
            .await?;

        Ok(response)
    }
}
