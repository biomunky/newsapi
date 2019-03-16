use newsapi::newsapi::newsapi::NewsAPI;
use std::env;

fn main() {

    let key = env::var("NEWSAPI_KEY").unwrap();

    let mut my_api = NewsAPI::new(key);

    my_api.get_sources();

    let r = my_api.run();

    println!("{:?}", r);

}