use booru::{danbooru::DanbooruClient, Client};

#[tokio::main]
async fn main() {
    let posts = DanbooruClient::builder()
        .limit(5)
        .build()
        .get_autocomplete("f")
        .await
        .expect("There was an error retrieving posts from the API");

    println!("{:?}", posts);
}
