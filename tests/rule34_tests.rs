mod rule34 {
    use booru::{
        client::{generic::*, rule34::Rule34Client, Client},
        model::rule34::Rule34Rating,
    };

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = Rule34Client::builder()
            .tag("kafuu_chino")
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_rating() {
        let posts = Rule34Client::builder()
            .tag("kafuu_chino")
            .rating(Rule34Rating::Questionable)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_sort() {
        let posts = Rule34Client::builder()
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
        let posts = Rule34Client::builder()
            .tag("kafuu_chino")
            .blacklist_tag(Rule34Rating::Explicit)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_limit() {
        let posts = Rule34Client::builder()
            .tag("kafuu_chino")
            .rating(Rule34Rating::Questionable)
            .limit(3)
            .build()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() == 3);
    }

    #[tokio::test]
    async fn get_posts_multiple_tags() {
        let posts = Rule34Client::builder()
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
        let posts = Rule34Client::builder()
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
        let posts = Rule34Client::builder().build().get_popular().await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_post_by_id() {
        let post = Rule34Client::builder().build().get_by_id(8725945).await;

        assert!(post.is_ok());
    }

    #[tokio::test]
    async fn get_by_page() {
        let posts = Rule34Client::builder()
            .tag("kafuu_chino")
            .limit(3)
            .build()
            .get_by_page(2)
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_autocomplete() {
        let posts = Rule34Client::builder()
            .limit(5)
            .build()
            .get_autocomplete("f")
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("explicit", Rule34Rating::Explicit.to_string());
        assert_eq!("questionable", Rule34Rating::Questionable.to_string());
        assert_eq!("safe", Rule34Rating::Safe.to_string());
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
    }
}
