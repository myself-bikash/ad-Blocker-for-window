use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkHealth {
    pub dns_ok: bool,
    pub internet_ok: bool,
    pub adapters: usize,
}

impl NetworkHealth {
    pub fn is_healthy(&self) -> bool {
        self.dns_ok && self.internet_ok && self.adapters > 0
    }
}

pub fn fail_open_policy() -> &'static str {
    "Fail open: if the service cannot initialize, normal connectivity is preserved and previous network settings are restored."
}
