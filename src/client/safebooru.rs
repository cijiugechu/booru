use itoa::Buffer;

use super::{Client, ClientBuilder, generic::AutoCompleteItem};
use crate::model::safebooru::SafebooruPost;

pub struct SafebooruClient(ClientBuilder<Self>);

impl From<ClientBuilder<Self>> for SafebooruClient {
    fn from(value: ClientBuilder<Self>) -> Self {
        Self(value)
    }
}

impl Client for SafebooruClient {
    type Post = SafebooruPost;

    const URL: &'static str = "https://safebooru.org";
    const SORT: &'static str = "sort:";

    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
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
            .json::<Vec<SafebooruPost>>()
            .await?;
        // FIXME: Assumes there is a post with the given id. Same is true for the
        // Gelbooru client.
        Ok(response
            .into_iter()
            .next()
            .expect("Requested an id that does not exist."))
    }

    #[tracing::instrument(skip(self))]
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let tags = builder.tags.join(" ");
        builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tags),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await
    }

    /// retrieve the most top rated posts
    #[tracing::instrument(skip(self))]
    async fn get_popular(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        builder
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
            .json::<Vec<SafebooruPost>>()
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_page(&self, page: u32) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let tags = builder.tags.join(" ");
        builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", Buffer::new().format(builder.limit)),
                ("tags", &tags),
                ("pid", Buffer::new().format(builder.limit * page)),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_autocomplete<Input: Into<String> + Send + std::fmt::Debug>(
        &self,
        input: Input,
    ) -> Result<Vec<AutoCompleteItem>, reqwest::Error> {
        let builder = &self.0;
        let url = "https://safebooru.org/autocomplete.php";
        let response = builder
            .client
            .get(url)
            .query(&[("q", &input.into())])
            .send()
            .await?
            .json::<Vec<AutoCompleteItem>>()
            .await?;

        Ok(response)
    }
}
