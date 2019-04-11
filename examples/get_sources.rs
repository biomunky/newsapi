use newsapi::newsapi::constants::Language;
use newsapi::newsapi::api::Client;
use newsapi::newsapi::payload::source::Sources;

use std::env;

fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    let sources = Client::new(key)
        .language(Language::English)
        .sources()
        .send::<Sources>();

    println!("{:?}", sources)
}
