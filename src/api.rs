use super::constants;
use super::error::NewsApiError;
use chrono::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Debug)]
pub struct Client {
    api_key: String,
    parameters: HashMap<String, String>,
    url: Option<String>,
}

impl Client {
    /// Returns a NewsAPI with given api key
    ///
    /// # Arguments
    ///
    /// * `api_key` - a string that holds the api, this will be used to set X-Api-Key.
    ///
    pub fn new(api_key: String) -> Client {
        Client {
            api_key,
            parameters: HashMap::new(),
            url: None,
        }
    }

    /// Build the 'fetch everything' url
    pub fn everything(&mut self) -> &mut Client {
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

        self.url = Some(self.build_url(constants::EVERYTHING_URL, allowed_params));
        self
    }

    /// Build the 'top_headlines' url
    pub fn top_headlines(&mut self) -> &mut Client {
        let allowed_params = vec!["q", "country", "category", "sources", "pageSize", "page"];
        self.url = Some(self.build_url(constants::TOP_HEADLINES_URL, allowed_params));
        self
    }

    /// Build the 'sources' url
    pub fn sources(&mut self) -> &mut Client {
        let allowed_params = vec!["category", "language", "country"];
        self.url = Some(self.build_url(constants::SOURCES_URL, allowed_params));
        self
    }

    /// Send the constructed URL to the newsapi server
    pub fn send<T>(&self) -> Result<T, NewsApiError>
    where
        T: DeserializeOwned,
    {
        if self.invalid_arguments_specified() {
            return Err(NewsApiError::InvalidParameterCombinationError);
        }

        match &self.url {
            Some(url) => {
                let body = Client::fetch_resource(url, &self.api_key)?;
                Ok(serde_json::from_str::<T>(&body)?)
            }
            None => Err(NewsApiError::UndefinedUrlError),
        }
    }

    fn build_url(&self, base_url: &str, allowed_params: Vec<&str>) -> String {
        let mut params: Vec<String> = vec![];
        for field in allowed_params {
            if let Some(value) = self.parameters.get(field) {
                params.push(format!("{}={}", field, value))
            }
        }

        let mut url = base_url.to_owned();

        if params.is_empty() {
            url
        } else {
            url.push_str("?");
            url.push_str(&params.join("&"));
            url
        }
    }

    fn invalid_arguments_specified(&self) -> bool {
        (self.parameters.contains_key("country") || self.parameters.contains_key("category"))
            && self.parameters.contains_key("sources")
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
        static CLIENT_USER_AGENT: &str = concat!(
            "rust-",
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        );

        // TODO: create a client that can be reused
        let client = reqwest::blocking::Client::builder()
            .user_agent(CLIENT_USER_AGENT)
            .build()?;

        let resp = client.get(url).header("X-Api-Key", api_key).send()?;

        if resp.status().is_success() {
            Ok(resp.text()?)
        } else {
            Err(Client::handle_api_error(
                resp.status().as_u16(),
                resp.text()?,
            ))
        }
    }

    /// A date and optional time for the oldest article allowed
    pub fn from(&mut self, from: &DateTime<Utc>) -> &mut Client {
        self.chronological_specification("from", from);
        self
    }

    /// A date and optional time for the newest article allowed.
    pub fn to(&mut self, to: &DateTime<Utc>) -> &mut Client {
        self.chronological_specification("to", to);
        self
    }

    fn chronological_specification(
        &mut self,
        operation: &str,
        dt_val: &DateTime<Utc>,
    ) -> &mut Client {
        let dt_format = "%Y-%m-%dT%H:%M:%S";
        self.parameters
            .insert(operation.to_owned(), dt_val.format(dt_format).to_string());
        self
    }

    ///  The domains
    /// (e.g. bbc.co.uk, techcrunch.com, engadget.com) to which search will be restricted.
    pub fn domains(&mut self, domains: Vec<&str>) -> &mut Client {
        self.manage_domains("domains", domains);
        self
    }

    /// The domains
    /// (e.g. bbc.co.uk, techcrunch.com, engadget.com) from which no stories will be present in the
    /// results.
    pub fn exclude_domains(&mut self, domains: Vec<&str>) -> &mut Client {
        self.manage_domains("excludeDomains", domains);
        self
    }

    fn manage_domains(&mut self, operation: &str, domains: Vec<&str>) -> &mut Client {
        self.parameters
            .insert(operation.to_owned(), domains.join(","));
        self
    }

    /// Narrow search to specific country
    pub fn country(&mut self, country: constants::Country) -> &mut Client {
        self.parameters.insert(
            "country".to_owned(),
            constants::COUNTRY_LOOKUP[country].to_string(),
        );
        self
    }

    /// Defaults to all categories - see constants.rs
    pub fn category(&mut self, category: constants::Category) -> &mut Client {
        let fmtd_category = format!("{:?}", category).to_lowercase();
        self.parameters.insert("category".to_owned(), fmtd_category);
        self
    }

    /// Use the /sources endpoint to locate these programmatically or look at the sources index.
    /// Note: you can't mix this param with the country or category params.
    /// This will be checked before calling the API but you can still get rekt!
    pub fn with_sources(&mut self, sources: String) -> &mut Client {
        self.parameters.insert("sources".to_owned(), sources);
        self
    }

    /// Keywords or phrases to search for.
    ///
    /// * Surround phrases with quotes (") for exact match.
    /// * Prepend words or phrases that must appear with a + symbol. Eg: +bitcoin
    /// * Prepend words that must not appear with a - symbol. Eg: -bitcoin
    /// * Alternatively you can use the AND / OR / NOT keywords, and optionally group these with parenthesis.
    ///   e.g.: crypto AND (ethereum OR litecoin) NOT bitcoin
    pub fn query(&mut self, query: &str) -> &mut Client {
        self.parameters.insert(
            "q".to_owned(),
            utf8_percent_encode(&query, NON_ALPHANUMERIC).to_string(),
        );
        self
    }

    pub fn page(&mut self, page: u32) -> &mut Client {
        self.parameters.insert("page".to_owned(), page.to_string());
        self
    }

    pub fn page_size(&mut self, size: u32) -> &mut Client {
        if size >= 1 && size <= 100 {
            self.parameters
                .insert("pageSize".to_owned(), size.to_string());
        }
        self
    }

    pub fn language(&mut self, language: constants::Language) -> &mut Client {
        self.parameters.insert(
            "language".to_owned(),
            constants::LANG_LOOKUP[language].to_string(),
        );
        self
    }

    pub fn sort_by(&mut self, sort_by: constants::SortMethod) -> &mut Client {
        self.parameters.insert(
            "sort_by".to_owned(),
            constants::SORT_METHOD_LOOKUP[sort_by].to_string(),
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_api_error() {
        let bad_request = Client::handle_api_error(400, "BadRequest".into());
        assert_eq!(bad_request.to_string(), "BadRequest: 400 => BadRequest");

        let generic_error =
            Client::handle_api_error(418, "Hyper Text Coffee Pot Control Protocol".into());
        assert_eq!(
            generic_error.to_string(),
            "GenericError: 418 => Hyper Text Coffee Pot Control Protocol"
        );
    }

    #[test]
    fn query() {
        let mut api = Client::new("123".to_owned());
        api.query("Ali loves the hoff NOT Baywatch");
        let encoded_param = api.parameters.get("q");
        assert_eq!(
            encoded_param,
            Some(&"Ali%20loves%20the%20hoff%20NOT%20Baywatch".to_string())
        );
    }

    #[test]
    fn build_url() {
        let mut api = Client::new("123".to_owned());
        api.language(constants::Language::English);
        api.country(constants::Country::UnitedStatesofAmerica);
        let expected = "https://newsapi.org/v2/sources?language=en&country=us".to_owned();
        let allowed_params = vec!["category", "language", "country"];
        let url = api.build_url(constants::SOURCES_URL, allowed_params);
        assert_eq!(expected, url);
    }

    #[test]
    fn domains() {
        let mut api = Client::new("123".to_owned());

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
        let mut api = Client::new("123".to_owned());
        assert_eq!(api.parameters.get("category"), None);
        api.category(constants::Category::Science);
        assert_eq!(api.parameters.get("category"), Some(&"science".to_owned()));
    }

    #[test]
    fn country() {
        let mut api = Client::new("123".to_owned());
        assert_eq!(api.parameters.get("country"), None);
        api.country(constants::Country::Germany);
        assert_eq!(api.parameters.get("country"), Some(&"de".to_owned()));
    }

    #[test]
    fn language() {
        let mut api = Client::new("123".to_owned());
        api.language(constants::Language::English);
        assert_eq!(api.parameters.get("language"), Some(&"en".to_owned()));
    }

    #[test]
    fn to_and_from() {
        let mut api = Client::new("123".to_owned());

        let from = Utc.ymd(2019, 7, 8).and_hms(9, 10, 11);
        let to = Utc.ymd(2019, 7, 9).and_hms(9, 10, 11);

        api.to(&to).from(&from);

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
        let api = Client::new("123".to_string());
        assert_eq!(api.api_key, "123".to_string());
    }

    #[test]
    fn page() {
        let mut api = Client::new("123".to_owned());
        api.page(20);
        assert_eq!(api.parameters.get("page"), Some(&"20".to_owned()));
    }

    #[test]
    fn page_size() {
        let mut api = Client::new("123".to_owned());
        assert_eq!(api.parameters.get("pageSize"), None);
        api.page_size(30);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
        api.page_size(400);
        assert_eq!(api.parameters.get("pageSize"), Some(&"30".to_owned()));
    }
}
