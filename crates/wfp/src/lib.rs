use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WfpController {
    pub enabled: bool,
    pub filters_loaded: usize,
}

impl WfpController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self) {
        self.enabled = true;
        self.filters_loaded = 1;
    }

    pub fn shutdown(&mut self) {
        self.enabled = false;
        self.filters_loaded = 0;
    }
}
