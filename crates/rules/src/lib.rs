use adblock_core::{DomainRule, RuleAction, RulePriority};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub rules: Vec<DomainRule>,
}

impl Default for RuleSet {
    fn default() -> Self {
        let rules = vec![
            DomainRule {
                domain: "googleads.g.doubleclick.net".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.googleads.g.doubleclick.net".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "pubads.g.doubleclick.net".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.pubads.g.doubleclick.net".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.doubleclick.net".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "googlesyndication.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.googlesyndication.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.google-analytics.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-analytics".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "adservice.google.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-ad-networks".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "scorecardresearch.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-trackers".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.scorecardresearch.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-trackers".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "quantserve.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-trackers".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "outbrain.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-trackers".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "taboola.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-trackers".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "ads.youtube.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-platform-ads".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "ads.spotify.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-platform-ads".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.ads.spotify.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-platform-ads".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "ads.twitch.tv".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-platform-ads".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.ads.twitch.tv".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-platform-ads".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.twitchads.tv".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-platform-ads".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "google-analytics.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-analytics".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "analytics.google.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-analytics".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "app-measurement.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-analytics".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.app-measurement.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "default-analytics".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "spotify.com".to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "safe-platform-allowlist".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "api.spotify.com".to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "safe-platform-allowlist".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "youtube.com".to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "safe-platform-allowlist".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "googlevideo.com".to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "safe-platform-allowlist".to_string(),
                enabled: true,
            },
        ];

        Self { rules }
    }
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: DomainRule) {
        self.rules.push(rule);
    }

    pub fn is_blocked(&self, domain: &str) -> bool {
        let result = adblock_core::evaluate_domain_rule(domain, &self.rules);
        matches!(result.action, RuleAction::Block)
    }
}

#[cfg(test)]
mod tests {
    use super::RuleSet;

    #[test]
    fn default_rules_block_common_ad_domains() {
        let rules = RuleSet::default();

        assert!(rules.is_blocked("googleads.g.doubleclick.net"));
        assert!(rules.is_blocked("pubads.g.doubleclick.net"));
        assert!(rules.is_blocked("googlesyndication.com"));
        assert!(rules.is_blocked("scorecardresearch.com"));
        assert!(rules.is_blocked("taboola.com"));
    }

    #[test]
    fn default_rules_keep_core_platform_domains_allowed() {
        let rules = RuleSet::default();

        assert!(!rules.is_blocked("spotify.com"));
        assert!(!rules.is_blocked("api.spotify.com"));
        assert!(!rules.is_blocked("youtube.com"));
        assert!(!rules.is_blocked("googlevideo.com"));
    }

    #[test]
    fn platform_specific_rule_groups_block_platform_ads() {
        let rules = RuleSet::default();

        assert!(rules.is_blocked("ads.youtube.com"));
        assert!(rules.is_blocked("ads.spotify.com"));
        assert!(rules.is_blocked("ads.twitch.tv"));
    }
}
