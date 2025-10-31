#[cfg(test)]
mod danbooru {
    use booru::{
        client::{Client, danbooru::DanbooruClient, generic::Sort},
        model::danbooru::DanbooruRating,
    };
    use reqwest::Proxy;

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_rating() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .rating(DanbooruRating::General)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_sort() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .sort(Sort::Rating)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_blacklist_tag() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .blacklist_tag(DanbooruRating::Explicit)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_limit() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .limit(3)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() == 3);
    }

    #[tokio::test]
    async fn get_posts_multiple_tags() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .tag("bangs")
            .limit(3)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_random_posts() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .random()
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_post_by_id() {
        let post = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .build()
            .get_by_id(9423)
            .await;

        assert!(post.is_ok());
        assert_eq!(
            "15a1b49c26f5c684807a2f0b838f9e4c",
            post.unwrap().md5.unwrap()
        );
    }

    #[tokio::test]
    async fn get_popular_posts() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .build()
            .get_popular()
            .await;

        assert!(posts.is_ok());
        // yes, it is empty in terms of testbooru
        assert!(posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_by_page() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .limit(5)
            .build()
            .get_by_page(2)
            .await;

        assert!(posts.is_ok());
    }

    #[tokio::test]
    async fn get_autocomplete() {
        let posts = DanbooruClient::builder()
            .limit(5)
            .build()
            .get_autocomplete("f")
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn builder_with_extra_http_config() {
        let http_client_builder =
            reqwest::ClientBuilder::new().proxy(Proxy::http("http://127.0.0.1:7890").unwrap());

        let posts = DanbooruClient::builder_with_http_client(http_client_builder)
            .default_url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("explicit", DanbooruRating::Explicit.to_string());
        assert_eq!("questionable", DanbooruRating::Questionable.to_string());
        assert_eq!("sensitive", DanbooruRating::Sensitive.to_string());
        assert_eq!("general", DanbooruRating::General.to_string());
    }

    #[test]
    fn parse_sort_tags() {
        assert_eq!("id", Sort::Id.to_string());
        assert_eq!("score", Sort::Score.to_string());
        assert_eq!("rating", Sort::Rating.to_string());
        assert_eq!("user", Sort::User.to_string());
        assert_eq!("height", Sort::Height.to_string());
        assert_eq!("width", Sort::Width.to_string());
        assert_eq!("source", Sort::Source.to_string());
        assert_eq!("updated", Sort::Updated.to_string());
    }
}
