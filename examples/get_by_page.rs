use booru::{danbooru::DanbooruClient, Client};

#[tokio::main]
async fn main() {
    let posts = DanbooruClient::builder()
        .tag("kafuu_chino")
        .limit(5)
        .build()
        .get_by_page(2)
        .await
        .expect("There was an error retrieving posts from the API");

    println!("{:?}", posts);
}
