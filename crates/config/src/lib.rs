use adblock_core::Profile;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service_name: String,
    pub profile: Profile,
    pub enabled: bool,
    pub upstream_dns: Vec<String>,
    pub allowlist: Vec<String>,
    pub blocklist: Vec<String>,
    pub log_path: String,
    pub auto_update: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            service_name: "AdBlockService".to_string(),
            profile: Profile::Balanced,
            enabled: true,
            upstream_dns: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string(), "9.9.9.9".to_string()],
            allowlist: vec![],
            blocklist: vec![],
            log_path: r"%ProgramData%\AdBlocker\logs".to_string(),
            auto_update: false,
        }
    }
}

impl ServiceConfig {
    pub fn from_default() -> Self {
        Self::default()
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("failed to read config file {path:?}: {e}"))?;

        let parsed: TomlConfig = toml::from_str(&content)
            .map_err(|e| format!("failed to parse config file {path:?}: {e}"))?;

        Ok(Self {
            service_name: parsed.service.name,
            profile: Profile::from_str(&parsed.service.profile).unwrap_or_default(),
            enabled: parsed.service.enabled,
            upstream_dns: parsed.network.upstream_dns,
            allowlist: parsed.filters.allowlist,
            blocklist: parsed.filters.blocklist,
            log_path: parsed.logging.path,
            auto_update: parsed.updates.allow_auto_update,
        })
    }
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    service: TomlService,
    network: TomlNetwork,
    filters: TomlFilters,
    logging: TomlLogging,
    updates: TomlUpdates,
}

#[derive(Debug, Deserialize)]
struct TomlService {
    name: String,
    profile: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct TomlNetwork {
    upstream_dns: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlFilters {
    allowlist: Vec<String>,
    blocklist: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlLogging {
    path: String,
}

#[derive(Debug, Deserialize)]
struct TomlUpdates {
    allow_auto_update: bool,
}
