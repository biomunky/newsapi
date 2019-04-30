extern crate time;

use chrono::prelude::*;
use newsapi::newsapi::api::Client;
use newsapi::newsapi::constants::{Category, Language, SortMethod};
use newsapi::newsapi::payload::article::Articles;

use time::Duration;

use std::env;

fn main() {
    let key = env::var("NEWSAPI_KEY").unwrap();

    let start_timestamp = Utc::now() - Duration::days(10);
    let end_timestamp = Utc::now();

    // create a client
    let mut c = Client::new(key);

    c
        // Search German news sources for articles
        .language(Language::German)
        // in the last ten days containing Trump and America
        .from(&start_timestamp)
        .to(&end_timestamp)
        // for articles that contain 'Trump' and 'America'
        .query("Trump America")
        // if they are in the 'general' category
        .category(Category::General)
        // Sort by the most popular articles
        .sort_by(SortMethod::Popularity)
        // get articles from the everything end point - this builds the url
        .everything();

    // debug print the current client status - you can see the URL that will be sent to the API
    println!("{:?}", c);

    // fire off a request to the endpoint and deserialize the results into an Article struct
    let articles = c.send::<Articles>();

    // print the results to the terminal
    println!("{:?}", articles);
}
