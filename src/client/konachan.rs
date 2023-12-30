use super::{generic::AutoCompleteItem, Client, ClientBuilder};
use crate::model::konachan::*;

use itoa::Buffer;
use reqwest::{header, header::HeaderMap};

fn get_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}

/// Client that sends requests to the Konachan API to retrieve the data.
pub struct KonachanClient(ClientBuilder<Self>);

impl From<ClientBuilder<Self>> for KonachanClient {
    fn from(value: ClientBuilder<Self>) -> Self {
        Self(value)
    }
}

impl Client for KonachanClient {
    type Post = KonachanPost;

    const URL: &'static str = "https://konachan.com";
    const SORT: &'static str = "order:";

    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/post.json"))
            .query(&[("tags", format!("id:{id}"))])
            .headers(get_headers())
            .send()
            .await?
            .json::<Vec<KonachanPost>>()
            .await?;

        Ok(response
            .into_iter()
            .next()
            .expect("Requested an id that does not exist."))
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/post.json"))
            .headers(get_headers())
            .query(&[
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<KonachanPost>>()
            .await?;

        Ok(response)
    }

    /// retrieve the most top rated posts
    async fn get_popular(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/post/popular_recent.json"))
            .headers(get_headers())
            .query(&[("limit", Buffer::new().format(builder.limit))])
            .send()
            .await?
            .json::<Vec<KonachanPost>>()
            .await?;

        Ok(response)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get_by_page(&self, page: u32) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/post.json"))
            .headers(get_headers())
            .query(&[
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tag_string),
                ("page", Buffer::new().format(page)),
            ])
            .send()
            .await?
            .json::<Vec<KonachanPost>>()
            .await?;

        Ok(response)
    }

    async fn get_autocomplete<Input: Into<String> + Send>(
        &self,
        _input: Input,
    ) -> Result<Vec<AutoCompleteItem>, reqwest::Error> {
        panic!("not implemented!");
    }
}
