use newsapi::api::NewsAPIClient;
use newsapi::constants::Language;
use newsapi::payload::source::Sources;

use std::env;

fn main() {
    let key_var = "NEWSAPI_KEY";

    let key = match env::var(key_var) {
        Ok(val) => val,
        Err(error) => panic!(
            "The '{:?}' environment variable is not set, please set it and try again.\n{:?}",
            key_var, error
        ),
    };

    // search for English language Sources
    let sources = NewsAPIClient::new(key)
        .language(Language::English)
        .sources()
        .send_sync::<Sources>();

    println!("{:?}", sources)
}
