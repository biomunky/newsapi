#[derive(Debug, Serialize, Deserialize)]
pub struct Articles {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: u64,
    articles: Vec<Article>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleSource {
    id: Option<String>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    source: ArticleSource,
    author: Option<String>,
    title: String,
    description: Option<String>,
    url: String,
    #[serde(rename = "urlToImage")]
    url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    published_at: String,
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn load_file(filename: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(filename);
        fs::read_to_string(d.as_path()).expect("Something went wrong reading the file")
    }

    #[test]
    fn deserialize_everything() {
        let contents = load_file("resources/example_everything.json");
        let articles: Articles = serde_json::from_str(&contents).unwrap();
        assert_eq!(articles.status, "ok");
        assert_eq!(articles.articles.len(), 2);
    }

    #[test]
    fn deserialize_headlines() {
        let contents = load_file("resources/example_headlines.json");
        let articles: Articles = serde_json::from_str(&contents).unwrap();
        assert_eq!(articles.total_results, 2);
        assert_eq!(articles.articles.len(), 2);
    }
}
