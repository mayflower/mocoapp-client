#![allow(dead_code)]
#[derive(Debug)]
struct MocoAppClient {
    api_key: String,
    domain: String,
}

impl MocoAppClient {
    pub fn new(api_key: String, domain: String) -> MocoAppClient {
        MocoAppClient { api_key, domain }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = MocoAppClient::new("key".to_string(), "domain".to_string());
        assert_eq!(client.api_key, "key");
        assert_eq!(client.domain, "domain");
    }
}
