use booru::{rule34::Rule34Client, Client};

#[tokio::main]
async fn main() {
    let posts = Rule34Client::builder()
        .tag("kafuu_chino")
        .limit(5)
        .build()
        .get()
        .await
        .expect("There was an error retrieving posts from the API");

    println!("{:?}", posts);
}
