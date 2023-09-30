use booru::{konachan::KonachanClient, Client};

#[tokio::main]
async fn main() {
    let posts = KonachanClient::builder()
        .tag("kafuu_chino")
        .limit(5)
        .build()
        .get()
        .await
        .expect("There was an error retrieving posts from the API");

    println!("{:?}", posts);
}
