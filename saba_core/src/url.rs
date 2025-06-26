use alloc::string::{String, ToString};

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
}
