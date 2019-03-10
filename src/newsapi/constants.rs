use std::collections::HashSet;
use std::iter::FromIterator;

#[allow(dead_code)]
pub const TOP_HEADLINES_URL: &'static str = "https://newsapi.org/v2/top-headlines";

#[allow(dead_code)]
pub const EVERYTHING_URL: &'static str = "https://newsapi.org/v2/everything";

#[allow(dead_code)]
pub const SOURCES_URL: &'static str = "https://newsapi.org/v2/sources";

lazy_static! {
    pub static ref COUNTRIES: HashSet<&'static str> = {
        let options = vec![
            "ae", "ar", "at", "au", "be", "bg", "br", "ca", "ch", "cn", "co", "cu", "cz", "de",
            "eg", "fr", "gb", "gr", "hk", "hu", "id", "ie", "il", "in", "it", "jp", "kr", "lt",
            "lv", "ma", "mx", "my", "ng", "nl", "no", "nz", "ph", "pl", "pt", "ro", "rs", "ru",
            "sa", "se", "sg", "si", "sk", "th", "tr", "tw", "ua", "us", "ve", "za",
        ];
        HashSet::from_iter(options)
    };
    pub static ref LANGUAGES: HashSet<&'static str> = {
        let options = vec![
            "ar", "en", "cn", "de", "es", "fr", "he", "it", "nl", "no", "pt", "ru", "sv", "ud",
        ];
        HashSet::from_iter(options)
    };
    pub static ref CATEGORIES: HashSet<&'static str> = {
        let options = vec![
            "business",
            "entertainment",
            "general",
            "health",
            "science",
            "sports",
            "technology",
        ];
        HashSet::from_iter(options)
    };
    pub static ref SORT_METHOD: HashSet<&'static str> = {
        let options = vec!["relevancy", "popularity", "publishedAt"];
        HashSet::from_iter(options)
    };
}
