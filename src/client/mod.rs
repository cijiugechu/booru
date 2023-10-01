use std::any::{Any, TypeId};

use async_trait::async_trait;

use self::{
    danbooru::DanbooruClient,
    gelbooru::GelbooruClient,
    generic::{Rating, Sort},
    konachan::KonachanClient,
    rule34::Rule34Client,
    safebooru::SafebooruClient,
};

pub mod danbooru;
pub mod gelbooru;
pub mod generic;
pub mod konachan;
pub mod rule34;
pub mod safebooru;

pub struct ClientBuilder<T: Client> {
    client: reqwest::Client,
    key: Option<String>,
    user: Option<String>,
    tags: Vec<String>,
    limit: u32,
    url: String,
    _marker: std::marker::PhantomData<T>,
}

#[async_trait]
pub trait Client: From<ClientBuilder<Self>> + Any {
    type Post;

    const URL: &'static str;
    const SORT: &'static str;

    fn builder() -> ClientBuilder<Self> {
        ClientBuilder::new()
    }

    fn builder_with_http_client(
        http_client_builder: reqwest::ClientBuilder,
    ) -> ClientBuilder<Self> {
        ClientBuilder {
            client: http_client_builder
                .build()
                .expect("fatal error: cannot build http client with wrong configuration!"),
            key: None,
            user: None,
            tags: vec![],
            limit: 100,
            url: Self::URL.to_string(),
            _marker: std::marker::PhantomData,
        }
    }

    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error>;
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error>;
    async fn get_popular(&self) -> Result<Vec<Self::Post>, reqwest::Error>;
    async fn get_by_page(&self, page: u32) -> Result<Vec<Self::Post>, reqwest::Error>;
}

impl<T: Client + Any> ClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            key: None,
            user: None,
            tags: vec![],
            limit: 100,
            url: T::URL.to_string(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Set the API key and User for the requests (optional)
    pub fn set_credentials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }

    /// Add a tag to the query
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        if TypeId::of::<DanbooruClient>() == TypeId::of::<T>() && self.tags.len() > 1 {
            panic!("Danbooru only allows 2 tags per query")
        }
        self.tags.push(tag.into());
        self
    }

    // REFACTOR: This can probably be cleaned up.
    /// Add the client compatible rating. Will panic if the rating is not compatible.
    pub fn rating<R: Into<Rating>>(mut self, rating: R) -> Self {
        let rating_tag = match rating.into() {
            Rating::Danbooru(rating) => {
                assert_eq!(
                    TypeId::of::<T>(),
                    TypeId::of::<DanbooruClient>(),
                    "{:?} `ClientBuilder` but tried to apply a Danbooru rating to it.",
                    TypeId::of::<T>(),
                );
                format!("rating:{}", rating)
            }
            Rating::Gelbooru(rating) => {
                assert_eq!(
                    TypeId::of::<T>(),
                    TypeId::of::<GelbooruClient>(),
                    "{:?} `ClientBuilder` but tried to apply a Gelbooru rating to it.",
                    TypeId::of::<T>(),
                );
                format!("rating:{}", rating)
            }
            Rating::Safebooru(rating) => {
                assert_eq!(
                    TypeId::of::<T>(),
                    TypeId::of::<SafebooruClient>(),
                    "{:?} `ClientBuilder` but tried to apply a Safebooru rating to it.",
                    TypeId::of::<T>(),
                );
                format!("rating:{}", rating)
            }
            Rating::Konachan(rating) => {
                assert_eq!(
                    TypeId::of::<T>(),
                    TypeId::of::<KonachanClient>(),
                    "{:?} `ClientBuilder` but tried to apply a Konachan rating to it.",
                    TypeId::of::<T>(),
                );
                format!("rating:{}", rating)
            }
            Rating::Rule34(rating) => {
                assert_eq!(
                    TypeId::of::<T>(),
                    TypeId::of::<Rule34Client>(),
                    "{:?} `ClientBuilder` but tried to apply a Rule34 rating to it.",
                    TypeId::of::<T>(),
                );
                format!("rating:{}", rating)
            }
        };
        self.tags.push(rating_tag);
        self
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Retrieves the posts in a random order
    pub fn random(mut self) -> Self {
        // let random_tag = match TypeId::of::<T>() {
        //     ClientType::Danbooru => "order:random",
        //     ClientType::Gelbooru => "sort:random",
        // };
        self.tags.push(format!("{}:random", T::SORT));
        self
    }

    /// Add a [`Sort`] to the query
    pub fn sort(mut self, order: Sort) -> Self {
        self.tags.push(format!("{}:{}", T::SORT, order));
        self
    }

    /// Blacklist a tag from the query
    pub fn blacklist_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(format!("-{}", tag.into()));
        self
    }

    /// Change the default url for the client
    pub fn default_url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    /// Convert the builder into the necessary client
    pub fn build(self) -> T {
        T::from(self)
    }
}

impl<T: Client + Any> Default for ClientBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
