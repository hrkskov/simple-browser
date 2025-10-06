use alloc::string::{String, ToString};
use alloc::vec::Vec;

const DEFAULT_HTTP_PORT: &str = "80";

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    pub fn get_port(&self) -> String {
        self.port.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_searchpart(&self) -> String {
        self.searchpart.clone()
    }

    /**
     * parses the URL and fills in the fields of the Url struct.
     * We assume the URL is of the form: http://host:port/path?searchpart
     */
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_supported_scheme() {
            return Err("Unsupported URL scheme".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    fn is_supported_scheme(&self) -> bool {
        self.url.starts_with("http://")
    }

    fn extract_host(&self) -> String {
        let parts: Vec<String> = self.get_domain_and_path();

        if let Some(index) = parts[0].find(':') {
            // Extract host before the colon
            parts[0][..index].to_string()
        } else {
            // No port specified, return the whole part
            parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let parts: Vec<String> = self.get_domain_and_path();

        if let Some(index) = parts[0].find(':') {
            parts[0][index + 1..].to_string()
        } else {
            DEFAULT_HTTP_PORT.to_string()
        }
    }

    fn extract_path(&self) -> String {
        let parts: Vec<String> = self.get_domain_and_path();

        if parts.len() < 2 {
            return "".to_string();
        }

        if let Some(index) = parts[1].find('?') {
            parts[1][..index].to_string()
        } else {
            parts[1].to_string()
        }
    }

    fn extract_searchpart(&self) -> String {
        let parts: Vec<String> = self.get_domain_and_path();

        if parts.len() < 2 {
            return "".to_string();
        }

        if let Some(index) = parts[1].find('?') {
            parts[1][index + 1..].to_string()
        } else {
            "".to_string()
        }
    }

    fn get_domain_and_path(&self) -> Vec<String> {
        self.url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .map(|s| s.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_new() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let url = Url::new(url_str.clone());
        assert_eq!(url.url, url_str);
        assert_eq!(url.host, "");
        assert_eq!(url.port, "");
        assert_eq!(url.path, "");
        assert_eq!(url.searchpart, "");
    }

    #[test]
    fn test_url_parse_valid() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let mut url = Url::new(url_str);
        let expected = Ok(Url {
            url: "http://example.com:8080/path?query=1".to_string(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "path".to_string(),
            searchpart: "query=1".to_string(),
        });

        assert_eq!(url.parse(), expected);
    }

    #[test]
    fn test_get_url() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let url = Url::new(url_str.clone());
        assert_eq!(url.get_url(), url_str);
    }

    #[test]
    fn test_get_host() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let mut url = Url::new(url_str);
        url.parse().unwrap();
        assert_eq!(url.get_host(), "example.com");
    }

    #[test]
    fn test_get_port() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let mut url = Url::new(url_str);
        url.parse().unwrap();
        assert_eq!(url.get_port(), "8080");
    }

    #[test]
    fn test_get_path() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let mut url = Url::new(url_str);
        url.parse().unwrap();
        assert_eq!(url.get_path(), "path");
    }

    #[test]
    fn test_get_searchpart() {
        let url_str = "http://example.com:8080/path?query=1".to_string();
        let mut url = Url::new(url_str);
        url.parse().unwrap();
        assert_eq!(url.get_searchpart(), "query=1");
    }
}
