use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    search_part: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            search_part: "".to_string(),
        }
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn search_part(&self) -> String {
        self.search_part.clone()
    }

    /// URLを解析する。
    ///
    /// サポートするスキームはHTTPのみ。
    /// HTTP以外の場合はエラーを返す。
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.search_part = self.extract_search_path();

        Ok(self.clone())
    }

    /// URLからホストを抽出する。
    ///
    /// http://example.com -> "example.com"
    /// http://example.com/path -> "example.com"
    /// http://example.com:8080 -> "example.com"
    fn extract_host(&mut self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    /// URLからポートを抽出する。
    ///
    /// http://example.com -> "80"
    /// http://example.com:8080 -> "8080"
    fn extract_port(&mut self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            // デフォルトのHTTPポート
            "80".to_string()
        }
    }

    /// URLからパスを抽出する。
    ///
    /// http://example.com -> ""
    /// http://example.com/path -> "path"
    fn extract_path(&mut self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_search_part: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        path_and_search_part[0].to_string()
    }

    /// URLからクエリパラををメをタを抽出する。
    fn extract_search_path(&mut self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_search_part: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        if path_and_search_part.len() < 2 {
            "".to_string()
        } else {
            path_and_search_part[1].to_string()
        }
    }

    fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();
        let parsed_url = Url::new(url).parse().unwrap();
        assert_eq!(parsed_url.url, "http://example.com");
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.search_part(), "");
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8888".to_string();
        let parsed_url = Url::new(url).parse().unwrap();
        assert_eq!(parsed_url.url, "http://example.com:8888");
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "8888");
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.search_part(), "");
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let parsed_url = Url::new(url).parse().unwrap();
        assert_eq!(parsed_url.url, "http://example.com:8888/index.html");
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "8888");
        assert_eq!(parsed_url.path(), "index.html");
        assert_eq!(parsed_url.search_part(), "");
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();
        let parsed_url = Url::new(url).parse().unwrap();
        assert_eq!(parsed_url.url, "http://example.com/index.html");
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "index.html");
        assert_eq!(parsed_url.search_part(), "");
    }

    #[test]
    fn test_url_host_port_path_search_query() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let parsed_url = Url::new(url).parse().unwrap();
        assert_eq!(parsed_url.url, "http://example.com:8888/index.html?a=123&b=456");
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "8888");
        assert_eq!(parsed_url.path(), "index.html");
        assert_eq!(parsed_url.search_part(), "a=123&b=456");
    }
}
