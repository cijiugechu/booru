use booru::{danbooru::DanbooruClient, Client};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let posts = DanbooruClient::builder()
        .limit(5)
        .build()
        .get_autocomplete("f")
        .await
        .expect("There was an error retrieving posts from the API");

    println!("{:?}", posts);
}
