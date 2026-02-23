use crate::constants::Category;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sources {
    pub status: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub category: Category,
    pub language: String,
    pub country: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn deserialize() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/example_sources.json");

        let contents =
            fs::read_to_string(d.as_path()).expect("Something went wrong reading the file");

        let sources: Sources = serde_json::from_str(&contents).unwrap();

        assert_eq!(sources.status, "ok");
        assert_eq!(sources.sources.len(), 4);
    }
}
