use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Newsapi {
    api_key: String,
    parameters: HashMap<String, String>,
}

impl Newsapi {
    pub fn new(api_key: String) -> Newsapi {
        Newsapi {
            api_key: api_key,
            parameters: HashMap::new(),
        }
    }

    //
    // Gitler:
    // There are crates for this but I don't want to use them yet
    //

    pub fn from(&mut self, from: DateTime<Utc>) -> &mut Newsapi {
        self.parameters.insert("from".to_owned(), from.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    pub fn to(&mut self, to: DateTime<Utc>) -> &mut Newsapi {
        self.parameters.insert("to".to_owned(), to.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    pub fn domains(&mut self, country: String) -> &mut Newsapi {
        self.parameters.insert("domains".to_owned(), country);
        self
    }

    pub fn exclude_domains(&mut self, country: String) -> &mut Newsapi {
        self.parameters.insert("exclude_domains".to_owned(), country);
        self
    }

    pub fn country(&mut self, country: String) -> &mut Newsapi {
        self.parameters.insert("country".to_owned(), country);
        self
    }

    pub fn category(&mut self, country: String) -> &mut Newsapi {
        self.parameters.insert("category".to_owned(), country);
        self
    }

    pub fn sources(&mut self, country: String) -> &mut Newsapi {
        self.parameters.insert("sources".to_owned(), country);
        self
    }

    pub fn query(&mut self, query: String) -> &mut Newsapi {
        self.parameters.insert("q".to_owned(), query);
        self
    }

    pub fn page(&mut self, page: i32) -> &mut Newsapi {
        self.parameters.insert("page".to_owned(), page.to_string());
        self
    }

    pub fn page_size(&mut self, size: i32) -> &mut Newsapi {
        self.parameters.insert("pageSize".to_owned(), size.to_string());
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_from() {
        let mut api = Newsapi::new("123".to_owned());

        let from = Utc.ymd(2019, 7, 8).and_hms(9, 10, 11);
        let to = Utc.ymd(2019, 7, 9).and_hms(9, 10, 11);

        api.to(to).from(from);

        assert_eq!(api.parameters.get("from"), Some(&"2019-07-08T09:10:11".to_owned()));
        assert_eq!(api.parameters.get("to"), Some(&"2019-07-09T09:10:11".to_owned()));
        
    }

    #[test]
    fn new() {
        let api = Newsapi::new("123".to_string());
        assert_eq!(api.api_key, "123".to_string());
    }

    #[test]
    fn update_page() {
        let mut api = Newsapi::new("123".to_owned());
        api.page(20);
        assert_eq!(api.parameters.get("page"), Some(&"20".to_owned()));
    }

    #[test]
    fn update_page_size() {
        let mut api = Newsapi::new("123".to_owned());
        assert_eq!(api.parameters.get("pageSize"), None);
        api.page_size(30);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
    }
}
