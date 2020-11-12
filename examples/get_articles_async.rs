use chrono::prelude::*;
use chrono::Duration;
use newsapi::api::Client;
use newsapi::constants::{Category, Language, SortMethod};
use newsapi::payload::article::Articles;

use std::env;

#[tokio::main]
async fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    let start_timestamp = Utc::now() - Duration::days(10);
    let end_timestamp = Utc::now();

    // create a client
    let mut c = Client::new(key);

    c
        // Search German news sources for articles
        .language(Language::German)
        // In the last ten days
        .from(&start_timestamp)
        .to(&end_timestamp)
        // For articles that contain 'Trump' and 'America'
        .query("Trump America")
        // And are categorized as 'general'
        .category(Category::General)
        // Sort by the most popular articles
        .sort_by(SortMethod::Popularity)
        // get articles from the everything endpoint - this builds the url
        .everything();

    // debug print the current client status - you can see the URL that will be sent to the API
    println!("{:?}", c);

    // fire off a request to the endpoint and deserialize the results into an Article struct
    let articles = c.send_async::<Articles>().await.unwrap();

    // print the results to the terminal
    println!("{:?}", articles);

    // access article status
    let status = articles.status;
    println!("{}", status);
}
