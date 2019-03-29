use super::constants;
use super::constants::Category;
use crate::newsapi::error::NewsApiError;
use chrono::prelude::*;
use std::collections::HashMap;

use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

#[derive(Debug)]
pub struct NewsAPI {
    api_key: String,
    parameters: HashMap<String, String>,
    url: Option<String>,
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
            url: None,
        }
    }

    pub fn get_everything(&mut self) -> &mut NewsAPI {
        let allowed_params = vec![
            "q",
            "sources",
            "domains",
            "excludeDomains",
            "from",
            "to",
            "language",
            "sortBy",
            "pageSize",
            "page",
        ];

        self.url = Some(self.build_url(allowed_params));
        self
    }

    pub fn get_top_headlines(&mut self) -> &mut NewsAPI {
        let allowed_params = vec!["q", "country", "category", "sources", "pageSize", "page"];
        self.url = Some(self.build_url(allowed_params));
        self
    }

    pub fn get_sources(&mut self) -> &mut NewsAPI {
        let allowed_params = vec!["category", "language", "country"];
        self.url = Some(self.build_url(allowed_params));
        self
    }

    fn build_url(&self, allowed_params: Vec<&str>) -> String {
        // TODO: can probably replace this with a fold
        let mut params: Vec<String> = vec![];
        for field in allowed_params {
            if let Some(value) = self.parameters.get(field) {
                params.push(format!("{}={}", field, value))
            }
        }

        let mut sources_url = constants::SOURCES_URL.to_owned();

        if params.is_empty() {
            sources_url
        } else {
            sources_url.push_str("?");
            sources_url.push_str(&params.join("&"));
            sources_url
        }
    }

    ///
    /// Attempt to fetch the constructed resource
    ///
    pub fn send(&self) -> Result<String, NewsApiError> {
        // TODO: validate all the parameters before firing off request //
        if (self.parameters.contains_key("country") || self.parameters.contains_key("category"))
            && self.parameters.contains_key("sources")
        {
            return Err(NewsApiError::InvalidParameterCombinationError);
        }

        match self
            .url
            .to_owned()
            .map(|u| NewsAPI::fetch_resource(&u, &self.api_key))
        {
            Some(s) => s,
            None => Err(NewsApiError::UndefinedUrlError),
        }
    }

    fn handle_api_error(error_code: u16, error_string: String) -> NewsApiError {
        match error_code {
            400 => NewsApiError::BadRequest {
                code: error_code,
                message: error_string,
            },
            401 => NewsApiError::Unauthorized {
                code: error_code,
                message: error_string,
            },
            429 => NewsApiError::TooManyRequests {
                code: error_code,
                message: error_string,
            },
            500 => NewsApiError::ServerError {
                code: error_code,
                message: error_string,
            },
            _ => NewsApiError::GenericError {
                code: error_code,
                message: error_string,
            },
        }
    }

    fn fetch_resource(url: &str, api_key: &str) -> Result<String, NewsApiError> {
        let client = reqwest::Client::new();
        let u = url.to_string();

        let mut resp = client.get(&u).header("X-Api-Key", api_key).send()?;

        if resp.status().is_success() {
            Ok(resp.text()?)
        } else {
            Err(NewsAPI::handle_api_error(
                resp.status().as_u16(),
                resp.text()?,
            ))
        }
    }

    /// A date and optional time for the oldest article allowed
    pub fn from(&mut self, from: DateTime<Utc>) -> &mut NewsAPI {
        self.parameters.insert(
            "from".to_owned(),
            from.format("%Y-%m-%dT%H:%M:%S").to_string(),
        );
        self
    }

    /// A date and optional time for the newest article allowed
    pub fn to(&mut self, to: DateTime<Utc>) -> &mut NewsAPI {
        self.parameters
            .insert("to".to_owned(), to.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    ///  A comma-seperated string of domains
    /// (eg bbc.co.uk, techcrunch.com, engadget.com) to restrict the search to.
    pub fn domains(&mut self, domains: Vec<&str>) -> &mut NewsAPI {
        self.parameters
            .insert("domains".to_owned(), domains.join(","));
        self
    }

    /// A comma-seperated string of domains
    /// (eg bbc.co.uk, techcrunch.com, engadget.com) to remove from the results.
    pub fn exclude_domains(&mut self, domains: Vec<&str>) -> &mut NewsAPI {
        self.parameters
            .insert("excludeDomains".to_owned(), domains.join(","));
        self
    }

    /// Defaults to all countries - see constants.rs
    pub fn country(&mut self, country: &str) -> &mut NewsAPI {
        if constants::COUNTRIES.contains(&*country) {
            self.parameters
                .insert("country".to_owned(), country.to_owned());
        }
        self
    }

    /// Defaults to all categories - see constants.rs
    pub fn category(&mut self, category: Category) -> &mut NewsAPI {
        let fmtd_category = format!("{:?}", category).to_lowercase();
        self.parameters.insert("category".to_owned(), fmtd_category);
        self
    }

    /// Use the /sources endpoint to locate these programmatically or look at the sources index.
    /// Note: you can't mix this param with the country or category params.
    /// This will be checked before calling the API but you can still get rekt!
    pub fn sources(&mut self, sources: String) -> &mut NewsAPI {
        self.parameters.insert("sources".to_owned(), sources);
        self
    }

    /// Keywords or phrases to search for.
    ///
    /// * Surround phrases with quotes (") for exact match.
    /// * Prepend words or phrases that must appear with a + symbol. Eg: +bitcoin
    /// * Prepend words that must not appear with a - symbol. Eg: -bitcoin
    /// * Alternatively you can use the AND / OR / NOT keywords, and optionally group these with parenthesis. Eg: crypto AND (ethereum OR litecoin) NOT bitcoin

    pub fn query(&mut self, query: String) -> &mut NewsAPI {
        self.parameters.insert(
            "q".to_owned(),
            utf8_percent_encode(&query, DEFAULT_ENCODE_SET).to_string(),
        );
        self
    }

    pub fn page(&mut self, page: u32) -> &mut NewsAPI {
        self.parameters.insert("page".to_owned(), page.to_string());
        self
    }

    pub fn page_size(&mut self, size: u32) -> &mut NewsAPI {
        if size >= 1 && size <= 100 {
            self.parameters
                .insert("pageSize".to_owned(), size.to_string());
        }
        self
    }

    pub fn language(&mut self, language: &str) -> &mut NewsAPI {
        if constants::LANGUAGES.contains(&*language) {
            self.parameters
                .insert("language".to_owned(), language.to_string());
        }
        self
    }

    pub fn sort_by(&mut self, sort_by: &str) -> &mut NewsAPI {
        if constants::SORT_METHOD.contains(&*sort_by) {
            self.parameters
                .insert("sort_by".to_owned(), sort_by.to_string());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_api_error() {
        let bad_request = NewsAPI::handle_api_error(400, "BadRequest".into());
        assert_eq!(bad_request.to_string(), "BadRequest: 400 => BadRequest");

        let generic_error =
            NewsAPI::handle_api_error(418, "Hyper Text Coffee Pot Control Protocol".into());
        assert_eq!(
            generic_error.to_string(),
            "GenericError: 418 => Hyper Text Coffee Pot Control Protocol"
        );
    }

    #[test]
    fn query() {
        let mut api = NewsAPI::new("123".to_owned());
        api.query("Ali loves the hoff NOT Baywatch".to_owned());
        let encoded_param = api.parameters.get("q");
        assert_eq!(
            encoded_param,
            Some(&"Ali%20loves%20the%20hoff%20NOT%20Baywatch".to_string())
        );
    }

    #[test]
    fn build_url() {
        let mut api = NewsAPI::new("123".to_owned());
        api.language("en");
        api.country("us");
        let expected = "https://newsapi.org/v2/sources?language=en&country=us".to_owned();
        let allowed_params = vec!["category", "language", "country"];
        let url = api.build_url(allowed_params);
        assert_eq!(expected, url);
    }

    #[test]
    fn domains() {
        let mut api = NewsAPI::new("123".to_owned());

        assert_eq!(api.parameters.get("domains"), None);
        assert_eq!(api.parameters.get("excludeDomains"), None);

        api.domains(vec!["www.bbc.co.uk"]);

        api.exclude_domains(vec!["www.facebook.com", "www.brexitbart.com"]);

        assert_eq!(
            api.parameters.get("domains"),
            Some(&"www.bbc.co.uk".to_owned())
        );

        assert_eq!(
            api.parameters.get("excludeDomains"),
            Some(&"www.facebook.com,www.brexitbart.com".to_owned())
        );
    }

    #[test]
    fn category() {
        let mut api = NewsAPI::new("123".to_owned());
        assert_eq!(api.parameters.get("category"), None);
        api.category(Category::Science);
        assert_eq!(api.parameters.get("category"), Some(&"science".to_owned()));
    }

    #[test]
    fn country() {
        let mut api = NewsAPI::new("123".to_owned());
        assert_eq!(api.parameters.get("country"), None);
        api.country("de");
        assert_eq!(api.parameters.get("country"), Some(&"de".to_owned()));
        api.country("HoffLand");
        assert_eq!(api.parameters.get("country"), Some(&"de".to_owned()));
    }

    #[test]
    fn language() {
        let mut api = NewsAPI::new("123".to_owned());
        api.language("en");
        assert_eq!(api.parameters.get("language"), Some(&"en".to_owned()));

        api.language("noSuchOption");
        assert_eq!(api.parameters.get("language"), Some(&"en".to_owned()));
    }

    #[test]
    fn to_and_from() {
        let mut api = NewsAPI::new("123".to_owned());

        let from = Utc.ymd(2019, 7, 8).and_hms(9, 10, 11);
        let to = Utc.ymd(2019, 7, 9).and_hms(9, 10, 11);

        api.to(to).from(from);

        assert_eq!(
            api.parameters.get("from"),
            Some(&"2019-07-08T09:10:11".to_owned())
        );
        assert_eq!(
            api.parameters.get("to"),
            Some(&"2019-07-09T09:10:11".to_owned())
        );
    }

    #[test]
    fn new() {
        let api = NewsAPI::new("123".to_string());
        assert_eq!(api.api_key, "123".to_string());
    }

    #[test]
    fn page() {
        let mut api = NewsAPI::new("123".to_owned());
        api.page(20);
        assert_eq!(api.parameters.get("page"), Some(&"20".to_owned()));
    }

    #[test]
    fn page_size() {
        let mut api = NewsAPI::new("123".to_owned());
        assert_eq!(api.parameters.get("pageSize"), None);
        api.page_size(30);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
        api.page_size(400);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
    }
}
