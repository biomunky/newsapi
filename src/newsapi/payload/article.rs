#[derive(Serialize, Deserialize)]
pub struct Articles {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: u64,
    articles: Vec<Article>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleSource {
    id: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize)]
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

    #[test]
    fn deserialize_everything() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/example_everything.json");

        let contents =
            fs::read_to_string(d.as_path()).expect("Something went wrong reading the file");

        let articles: Articles = serde_json::from_str(&contents).unwrap();

        assert_eq!(articles.status, "ok");
        assert_eq!(articles.articles.len(), 2);
    }

    #[test]
    fn deserialize_headlines() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/example_headlines.json");

        let contents =
            fs::read_to_string(d.as_path()).expect("Something went wrong reading the file");

        let articles: Articles = serde_json::from_str(&contents).unwrap();
        assert_eq!(articles.total_results, 2);
        assert_eq!(articles.articles.len(), 2);
    }
}
