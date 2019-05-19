use newsapi::api::Client;
use newsapi::constants::Language;
use newsapi::payload::source::Sources;

use std::env;

fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    // search for English language Sources
    let sources = Client::new(key)
        .language(Language::English)
        .sources()
        .send::<Sources>();

    println!("{:?}", sources)
}
