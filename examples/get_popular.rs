use booru::{Client, danbooru::DanbooruClient};

#[tokio::main]
async fn main() {
    let posts = DanbooruClient::builder()
        .limit(5)
        .build()
        .get_popular()
        .await
        .expect("There was an error retrieving posts from the API");

    println!("{:?}", posts);
}
