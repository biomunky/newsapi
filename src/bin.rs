use newsapi::newsapi::constants::Language;
use newsapi::newsapi::newsapi::NewsAPI;
use newsapi::newsapi::payload::source::Sources;

use std::env;

fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    let mut my_api = NewsAPI::new(key);
    let r = my_api
        .language(Language::English)
        .sources()
        .send::<Sources>();

    println!("{:?}", r)
}
