use itoa::Buffer;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

use super::{Client, ClientBuilder, generic::AutoCompleteItem};
use crate::model::gelbooru::*;

/// Client that sends requests to the Gelbooru API to retrieve the data.
pub struct GelbooruClient(ClientBuilder<Self>);

impl From<ClientBuilder<Self>> for GelbooruClient {
    fn from(value: ClientBuilder<Self>) -> Self {
        Self(value)
    }
}

impl GelbooruClient {
    fn default_query(&self) -> Vec<(&str, &str)> {
        if let (Some(api_ref), Some(user_ref)) = (&self.0.key, &self.0.user) {
            let api = api_ref.as_str();
            let user = user_ref.as_str();
            return vec![("api_key", api), ("user_id", user)];
        }
        Vec::new()
    }

    async fn get<T: DeserializeOwned, U: IntoUrl>(
        &self,
        url: U,
        query: &[(&str, &str)],
    ) -> Result<T, reqwest::Error> {
        let mut e_query = self.default_query();
        e_query.extend(query);
        self.0
            .client
            .get(url)
            .query(&e_query)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
    }
}

impl Client for GelbooruClient {
    type Post = GelbooruPost;

    const URL: &'static str = "https://gelbooru.com";
    const SORT: &'static str = "sort:";

    /// Directly get a post by its unique Id
    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: u32) -> Result<GelbooruPost, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();

        let response = self
            .get::<GelbooruResponse, _>(
                format!("{url}/index.php"),
                &[
                    ("page", "dapi"),
                    ("s", "post"),
                    ("q", "index"),
                    ("id", Buffer::new().format(id)),
                    ("json", "1"),
                ],
            )
            .await?;
        Ok(response.posts[0].clone())
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    #[tracing::instrument(skip(self))]
    async fn get(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let tag_string = builder.tags.join(" ");
        let response = self
            .get::<GelbooruResponse, _>(
                format!("{url}/index.php"),
                &[
                    ("page", "dapi"),
                    ("s", "post"),
                    ("q", "index"),
                    ("limit", Buffer::new().format(builder.limit)),
                    ("tags", &tag_string),
                    ("json", "1"),
                ],
            )
            .await?;

        Ok(response.posts)
    }

    /// retrieve the most top rated posts
    #[tracing::instrument(skip(self))]
    async fn get_popular(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = self
            .get::<GelbooruResponse, _>(
                format!("{url}/index.php"),
                &[
                    ("page", "dapi"),
                    ("s", "post"),
                    ("q", "index"),
                    ("limit", Buffer::new().format(builder.limit)),
                    ("tags", "sort:score:desc"),
                    ("json", "1"),
                ],
            )
            .await?;

        Ok(response.posts)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    #[tracing::instrument(skip(self))]
    async fn get_by_page(&self, page: u32) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let tag_string = builder.tags.join(" ");
        let response = self
            .get::<GelbooruResponse, _>(
                format!("{url}/index.php"),
                &[
                    ("page", "dapi"),
                    ("s", "post"),
                    ("q", "index"),
                    ("limit", Buffer::new().format(builder.limit)),
                    ("tags", &tag_string),
                    ("pid", Buffer::new().format(builder.limit * page)),
                    ("json", "1"),
                ],
            )
            .await?;

        Ok(response.posts)
    }

    #[tracing::instrument(skip(self))]
    async fn get_autocomplete<Input: Into<String> + Send + std::fmt::Debug>(
        &self,
        input: Input,
    ) -> Result<Vec<AutoCompleteItem>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = self
            .get::<Vec<AutoCompleteItem>, _>(
                format!("{url}/index.php"),
                &[
                    ("page", "autocomplete2"),
                    ("type", "tag_query"),
                    ("term", &input.into()),
                    ("limit", Buffer::new().format(builder.limit)),
                ],
            )
            .await?;

        Ok(response)
    }
}
