#[derive(Debug, Serialize, Deserialize)]
pub struct Articles {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: usize,
    articles: Vec<Article>,
}

impl Articles {
    pub fn sources(&self) -> Vec<&ArticleSource> {
        self.articles.iter().map(|a| &a.source).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleSource {
    id: Option<String>,
    name: String,
}

impl PartialEq for ArticleSource {
    fn eq(&self, other: &ArticleSource) -> bool {
        self.id == other.id && self.name == other.name
    }
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

    fn read_articles(filename: &str) -> Articles {
        let contents = load_file(filename);
        let articles: Articles = serde_json::from_str(&contents).unwrap();
        articles
    }

    #[test]
    fn article_sources() {
        let articles = read_articles("resources/example_everything.json");

        let wired = ArticleSource {
            id: Some("wired".into()),
            name: "Wired".into(),
        };

        let engadget = ArticleSource {
            id: Some("engadget".into()),
            name: "Engadget".into(),
        };

        let expected_sources = vec![&wired, &engadget];
        let sources = articles.sources();
        assert_eq!(sources, expected_sources);
    }

    #[test]
    fn deserialize_everything() {
        let articles = read_articles("resources/example_everything.json");
        assert_eq!(articles.status, "ok");
        assert_eq!(articles.articles.len(), 2);
        assert_eq!(articles.total_results, articles.articles.len());
    }

    #[test]
    fn deserialize_headlines() {
        let articles = read_articles("resources/example_headlines.json");
        assert_eq!(articles.total_results, 2);
        assert_eq!(articles.articles.len(), 2);
    }
}
