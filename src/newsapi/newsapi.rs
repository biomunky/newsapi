use std::collections::HashMap;
use chrono::prelude::*;
use super::constants;

#[derive(Debug)]
pub struct NewsAPI {
    api_key: String,
    parameters: HashMap<String, String>,
}

impl NewsAPI {
    /// Returns a NewsAPI with given api key
    ///
    /// # Arguments
    ///
    /// * `api_key` - a string that holds the api, this will be used to set X-Api-Key.
    ///
    pub fn new(api_key: String) -> NewsAPI {
        NewsAPI {
            api_key: api_key,
            parameters: HashMap::new(),
        }
    }

    //
    // Gitler:
    // There are crates for this but I don't want to use them yet
    //

    ///
    /// A date and optional time for the oldest article allowed
    ///
    pub fn from(&mut self, from: DateTime<Utc>) -> &mut NewsAPI {
        self.parameters.insert("from".to_owned(), from.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    ///
    /// A date and optional time for the newest article allowed
    ///
    pub fn to(&mut self, to: DateTime<Utc>) -> &mut NewsAPI {
        self.parameters.insert("to".to_owned(), to.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    pub fn domains(&mut self, country: String) -> &mut NewsAPI {
        self.parameters.insert("domains".to_owned(), country);
        self
    }

    pub fn exclude_domains(&mut self, country: String) -> &mut NewsAPI {
        self.parameters.insert("exclude_domains".to_owned(), country);
        self
    }

    pub fn country(&mut self, country: String) -> &mut NewsAPI {
        self.parameters.insert("country".to_owned(), country);
        self
    }

    pub fn category(&mut self, country: String) -> &mut NewsAPI {
        self.parameters.insert("category".to_owned(), country);
        self
    }

    pub fn sources(&mut self, country: String) -> &mut NewsAPI {
        self.parameters.insert("sources".to_owned(), country);
        self
    }

    // TODO: needs to urlencode the query string
    /// Keywords or phrases to search for.
    ///
    /// * Surround phrases with quotes (") for exact match.
    /// * Prepend words or phrases that must appear with a + symbol. Eg: +bitcoin
    /// * Prepend words that must not appear with a - symbol. Eg: -bitcoin
    /// * Alternatively you can use the AND / OR / NOT keywords, and optionally group these with parenthesis. Eg: crypto AND (ethereum OR litecoin) NOT bitcoin
    ///
    pub fn query(&mut self, query: String) -> &mut NewsAPI {
        self.parameters.insert("q".to_owned(), query);
        self
    }

    pub fn page(&mut self, page: u32) -> &mut NewsAPI {
        self.parameters.insert("page".to_owned(), page.to_string());
        self
    }

    pub fn page_size(&mut self, size: u32) -> &mut NewsAPI {
        if size > 1 && size <= 100 {
            self.parameters.insert("pageSize".to_owned(), size.to_string());
        }
        self
    }

    pub fn language(&mut self, language: String) -> &mut NewsAPI {
        if constants::LANGUAGES.contains(&*language) {
            self.parameters.insert("language".to_owned(), language);
        }
        self
    }

    pub fn sort_by(&mut self, sort_by: String) -> &mut NewsAPI {
        self.parameters.insert("sort_by".to_owned(), sort_by);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language() {
        let mut api = NewsAPI::new("123".to_owned());
        api.language("en".to_owned());
        assert_eq!(api.parameters.get("language"), Some(&"en".to_owned()));

        api.language("noSuchOption".to_owned());
        assert_eq!(api.parameters.get("language"), Some(&"en".to_owned()));
    }

    #[test]
    fn to_and_from() {
        let mut api = NewsAPI::new("123".to_owned());

        let from = Utc.ymd(2019, 7, 8).and_hms(9, 10, 11);
        let to = Utc.ymd(2019, 7, 9).and_hms(9, 10, 11);

        api.to(to).from(from);

        assert_eq!(api.parameters.get("from"), Some(&"2019-07-08T09:10:11".to_owned()));
        assert_eq!(api.parameters.get("to"), Some(&"2019-07-09T09:10:11".to_owned()));
        
    }

    #[test]
    fn new() {
        let api = NewsAPI::new("123".to_string());
        assert_eq!(api.api_key, "123".to_string());
    }

    #[test]
    fn update_page() {
        let mut api = NewsAPI::new("123".to_owned());
        api.page(20);
        assert_eq!(api.parameters.get("page"), Some(&"20".to_owned()));
    }

    #[test]
    fn update_page_size() {
        let mut api = NewsAPI::new("123".to_owned());
        assert_eq!(api.parameters.get("pageSize"), None);
        api.page_size(30);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
        api.page_size(400);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
    }
}
