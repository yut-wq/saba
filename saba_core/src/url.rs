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
