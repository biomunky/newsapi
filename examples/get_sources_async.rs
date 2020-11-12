use newsapi::api::Client;
use newsapi::constants::Language;
use newsapi::payload::source::Sources;

use std::env;

#[tokio::main]
async fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    // search for English language Sources
    let sources = Client::new(key)
        .language(Language::English)
        .sources()
        .send_async::<Sources>()
        .await;

    println!("{:?}", sources)
}
