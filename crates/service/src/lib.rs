use adblock_config::ServiceConfig;
use adblock_core::{
    evaluate_domain_rule, DomainRule, Profile, RuleAction, RuleEvaluation, RulePriority,
};
use adblock_diagnostics::DiagnosticsReport;
use adblock_dns::DnsFilter;
use adblock_network::NetworkHealth;
use adblock_wfp::WfpController;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceStatus {
    pub enabled: bool,
    pub running: bool,
    pub profile: String,
    pub rules_loaded: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdBlockService {
    pub config: ServiceConfig,
    pub dns: DnsFilter,
    pub network: NetworkHealth,
    pub wfp: WfpController,
    pub diagnostics: DiagnosticsReport,
    pub status: ServiceStatus,
}

impl Default for AdBlockService {
    fn default() -> Self {
        let dns = DnsFilter::default();
        let rules_loaded = dns.rules.rules.len();

        Self {
            config: ServiceConfig::default(),
            dns,
            network: NetworkHealth {
                dns_ok: true,
                internet_ok: true,
                adapters: 1,
            },
            wfp: WfpController::new(),
            diagnostics: DiagnosticsReport {
                service_status: true,
                dns_status: true,
                network_status: true,
                wfp_status: true,
                ipc_status: true,
            },
            status: ServiceStatus {
                enabled: true,
                running: true,
                profile: "Balanced".to_string(),
                rules_loaded,
            },
        }
    }
}

impl AdBlockService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.status.enabled
    }

    pub fn set_profile(&mut self, profile: Profile) {
        self.config.profile = profile.clone();
        self.status.profile = match profile {
            Profile::Balanced => "Balanced".to_string(),
            Profile::Strict => "Strict".to_string(),
            Profile::Aggressive => "Aggressive".to_string(),
            Profile::Custom => "Custom".to_string(),
        };
    }

    pub fn refresh_status(&mut self) {
        self.status.rules_loaded = self.dns.rules.rules.len();
    }

    pub fn start(&mut self) {
        self.config.enabled = true;
        self.status.enabled = true;
        self.status.running = true;
        self.refresh_status();
        self.wfp.initialize();
    }

    pub fn enable(&mut self) {
        self.start();
    }

    pub fn disable(&mut self) {
        self.config.enabled = false;
        self.status.enabled = false;
        self.status.running = false;
        self.wfp.shutdown();
    }

    pub fn stop(&mut self) {
        self.disable();
    }

    pub fn start_service_loop(&mut self, stop_after: std::time::Duration) {
        self.start();
        std::thread::sleep(stop_after);
        self.stop();
    }

    pub fn update_runtime_rules(&mut self) {
        let defaults = adblock_rules::RuleSet::default();
        let mut merged = defaults.rules;

        for domain in &self.config.allowlist {
            merged.push(DomainRule {
                domain: domain.trim().to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "config-allowlist".to_string(),
                enabled: true,
            });
        }

        for domain in &self.config.blocklist {
            merged.push(DomainRule {
                domain: domain.trim().to_string(),
                action: RuleAction::Block,
                priority: RulePriority::UserBlocklist,
                source: "config-blocklist".to_string(),
                enabled: true,
            });
        }

        self.dns.rules = adblock_rules::RuleSet { rules: merged };
        self.refresh_status();
    }

    pub fn evaluate_domain(&self, domain: &str) -> RuleEvaluation {
        evaluate_domain_rule(domain, &self.dns.rules.rules)
    }

    pub fn resolve_domain(&mut self, domain: &str) -> bool {
        if !self.is_enabled() {
            self.dns.cache_insert(domain);
            return false;
        }

        let blocked = self.is_domain_blocked(domain);
        self.dns.cache_insert(domain);
        self.refresh_status();
        blocked
    }

    pub fn is_domain_blocked(&self, domain: &str) -> bool {
        matches!(self.evaluate_domain(domain).action, adblock_core::RuleAction::Block)
    }

    pub fn persist(&self, path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("failed to create state dir: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("failed to encode service state: {e}"))?;
        fs::write(path, json).map_err(|e| format!("failed to write service state: {e}"))?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let json = fs::read_to_string(path)
            .map_err(|e| format!("failed to read service state: {e}"))?;
        let service: AdBlockService = serde_json::from_str(&json)
            .map_err(|e| format!("failed to parse service state: {e}"))?;
        Ok(service)
    }

    pub fn status_snapshot(&self) -> ServiceStatus {
        self.status.clone()
    }
}

pub fn service_banner() -> &'static str {
    "AdBlockService is the Windows background service responsible for persistent rule evaluation and fail-open network safety."
}

pub fn default_state_path() -> PathBuf {
    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()));
    let mut path = PathBuf::from(local_app_data);
    path.push("AdBlocker");
    path.push("service_state.json");
    path
}

#[cfg(test)]
mod tests {
    use super::AdBlockService;
    use std::time::Duration;

    #[test]
    fn start_and_stop_transition_updates_runtime_state() {
        let mut service = AdBlockService::new();

        service.disable();
        assert!(!service.is_enabled());
        assert!(!service.status_snapshot().running);

        service.start();
        assert!(service.is_enabled());
        assert!(service.status_snapshot().running);

        service.start_service_loop(Duration::from_millis(1));
        assert!(!service.is_enabled());
        assert!(!service.status_snapshot().running);
    }

    #[test]
    fn evaluate_known_blocked_and_unknown_allowed_domains() {
        let service = AdBlockService::new();

        let blocked = service.evaluate_domain("ads.youtube.com");
        let allowed = service.evaluate_domain("example.net");

        assert!(matches!(blocked.action, adblock_core::RuleAction::Block));
        assert!(matches!(allowed.action, adblock_core::RuleAction::Allow));
        assert!(service.is_domain_blocked("ads.youtube.com"));
        assert!(!service.is_domain_blocked("example.net"));
    }

    #[test]
    fn runtime_dns_filter_enforces_default_block_rules() {
        let mut service = AdBlockService::new();

        assert!(service.resolve_domain("ads.youtube.com"));
        assert!(service.dns.cache_lookup("ads.youtube.com"));
        assert!(!service.resolve_domain("youtube.com"));
    }

    #[test]
    fn status_tracks_active_rule_count() {
        let service = AdBlockService::new();

        let snapshot = service.status_snapshot();
        assert_eq!(snapshot.rules_loaded, service.dns.rules.rules.len());
    }

    #[test]
    fn runtime_rules_reload_from_config_and_override_defaults() {
        let mut service = AdBlockService::new();
        service.config.allowlist.push("youtube.com".to_string());
        service.config.blocklist.push("ads.youtube.com".to_string());

        service.update_runtime_rules();

        assert!(!service.is_domain_blocked("youtube.com"));
        assert!(service.is_domain_blocked("ads.youtube.com"));
    }
}
