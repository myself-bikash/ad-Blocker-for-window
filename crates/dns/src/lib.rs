use adblock_core::{DomainRule, RulePriority};
use adblock_rules::RuleSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsFilter {
    pub upstream: Vec<String>,
    pub rules: RuleSet,
    pub cache: Vec<String>,
}

impl Default for DnsFilter {
    fn default() -> Self {
        Self {
            upstream: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string(), "9.9.9.9".to_string()],
            rules: RuleSet::default(),
            cache: vec![],
        }
    }
}

impl DnsFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: DomainRule) {
        self.rules.add_rule(rule);
    }

    pub fn should_block(&self, domain: &str) -> bool {
        self.rules.is_blocked(domain)
    }

    pub fn cache_lookup(&self, domain: &str) -> bool {
        self.cache.iter().any(|entry| entry.eq_ignore_ascii_case(domain))
    }

    pub fn cache_insert(&mut self, domain: &str) {
        let clean = domain.trim();
        if !clean.is_empty() && !self.cache_lookup(clean) {
            self.cache.push(clean.to_string());
        }
    }

    pub fn precedence() -> Vec<RulePriority> {
        vec![
            RulePriority::SystemSafety,
            RulePriority::UserAllowlist,
            RulePriority::UserBlocklist,
            RulePriority::TrustedLocal,
            RulePriority::DownloadedLists,
            RulePriority::NetworkRules,
            RulePriority::DefaultAllow,
        ]
    }
}

pub fn dns_operation_summary() -> &'static str {
    "Local DNS filtering is configured with upstream servers, local rules, and safe fail-open behavior."
}
