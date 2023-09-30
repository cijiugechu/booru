mod konachan {
    use booru::{
        client::{generic::*, konachan::KonachanClient, Client},
        model::konachan::KonachanRating,
    };

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_rating() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .rating(KonachanRating::Questionable)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_sort() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .sort(Sort::Score)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_blacklist_tag() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .blacklist_tag(KonachanRating::Explicit)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_limit() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .rating(KonachanRating::Safe)
            .limit(3)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() == 3);
    }

    #[tokio::test]
    async fn get_posts_multiple_tags() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .tag("chinomaron")
            .limit(3)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_random_posts() {
        let posts = KonachanClient::builder()
            .tag("kafuu_chino")
            .random()
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_popular_posts() {
        let posts = KonachanClient::builder().build().get_popular().await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_post_by_id() {
        let post = KonachanClient::builder().build().get_by_id(363206).await;

        assert!(post.is_ok());
        assert_eq!(
            "7e50387c6649909a21bcef4958ea49b0",
            post.unwrap().md5.unwrap()
        );
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("explicit", KonachanRating::Explicit.to_string());
        assert_eq!("questionable", KonachanRating::Questionable.to_string());
        assert_eq!("safe", KonachanRating::Safe.to_string());
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
