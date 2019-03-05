#[derive(Debug)]
pub struct Newsapi {
    api_key: String,
}

impl Newsapi {
    pub fn new(api_key: String) -> Newsapi {
        Newsapi { api_key: api_key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let api = Newsapi::new("123".to_string());
        assert_eq!(api.api_key, "123".to_string());
    }
}
