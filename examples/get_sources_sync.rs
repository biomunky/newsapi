use newsapi::api::NewsAPIClient;
use newsapi::constants::Language;
use newsapi::payload::source::Sources;

use std::env;

fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    // search for English language Sources
    let sources = NewsAPIClient::new(key)
        .language(Language::English)
        .sources()
        .send_sync::<Sources>();

    println!(
        "{}",
        serde_json::to_string_pretty(&sources.unwrap()).unwrap()
    );
}
