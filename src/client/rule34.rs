use async_trait::async_trait;
use itoa::Buffer;

use super::{generic::AutoCompleteItem, Client, ClientBuilder};
use crate::model::rule34::*;

/// Client that sends requests to the Rule34 API to retrieve the data.
pub struct Rule34Client(ClientBuilder<Self>);

impl From<ClientBuilder<Self>> for Rule34Client {
    fn from(value: ClientBuilder<Self>) -> Self {
        Self(value)
    }
}

#[async_trait]
impl Client for Rule34Client {
    type Post = Rule34Post;

    const URL: &'static str = "https://api.rule34.xxx";
    const SORT: &'static str = "sort:";

    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<Rule34Post, reqwest::Error> {
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
            .json::<Vec<Rule34Post>>()
            .await?;

        Ok(response[0].clone())
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<Rule34Post>, reqwest::Error> {
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
            .json::<Vec<Rule34Post>>()
            .await?;

        Ok(response)
    }

    /// retrieve the most top rated posts
    async fn get_popular(&self) -> Result<Vec<Rule34Post>, reqwest::Error> {
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
            .json::<Vec<Rule34Post>>()
            .await?;

        Ok(response)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get_by_page(&self, page: u32) -> Result<Vec<Rule34Post>, reqwest::Error> {
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
            .json::<Vec<Rule34Post>>()
            .await?;

        Ok(response)
    }

    async fn get_autocomplete<Input: Into<String> + Send>(
        &self,
        input: Input,
    ) -> Result<Vec<AutoCompleteItem>, reqwest::Error> {
        let builder = &self.0;
        let url = "https://rule34.xxx/public/autocomplete.php";
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
