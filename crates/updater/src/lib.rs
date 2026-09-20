use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FilterListUpdate {
    pub source_url: String,
    pub target_path: PathBuf,
    pub validated: bool,
}

impl FilterListUpdate {
    pub fn new(source_url: impl Into<String>, target_path: impl Into<String>) -> Self {
        Self {
            source_url: source_url.into(),
            target_path: PathBuf::from(target_path.into()),
            validated: false,
        }
    }

    pub fn validate(&mut self) -> bool {
        self.validated = !self.source_url.trim().is_empty() && self.target_path.as_os_str().len() > 0;
        self.validated
    }
}

pub fn update_policy() -> &'static str {
    "Downloaded filter lists are validated and atomically swapped only after successful parsing and verification."
}
