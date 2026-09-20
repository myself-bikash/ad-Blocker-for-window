use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Profile {
    Balanced,
    Strict,
    Aggressive,
    Custom,
}

impl Default for Profile {
    fn default() -> Self {
        Self::Balanced
    }
}

impl Profile {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Balanced => "Balanced",
            Self::Strict => "Strict",
            Self::Aggressive => "Aggressive",
            Self::Custom => "Custom",
        }
    }
}

impl FromStr for Profile {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "Balanced" | "balanced" => Ok(Self::Balanced),
            "Strict" | "strict" => Ok(Self::Strict),
            "Aggressive" | "aggressive" => Ok(Self::Aggressive),
            "Custom" | "custom" => Ok(Self::Custom),
            _ => Err(format!("unknown profile: {value}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RulePriority {
    SystemSafety,
    UserAllowlist,
    UserBlocklist,
    TrustedLocal,
    DownloadedLists,
    NetworkRules,
    DefaultAllow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DomainRule {
    pub domain: String,
    pub action: RuleAction,
    pub priority: RulePriority,
    pub source: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleAction {
    Allow,
    Block,
    Ignore,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleEvaluation {
    pub action: RuleAction,
    pub priority: RulePriority,
    pub matched_rule: Option<String>,
}

fn priority_rank(priority: &RulePriority) -> u8 {
    match priority {
        RulePriority::SystemSafety => 0,
        RulePriority::UserAllowlist => 1,
        RulePriority::UserBlocklist => 2,
        RulePriority::TrustedLocal => 3,
        RulePriority::DownloadedLists => 4,
        RulePriority::NetworkRules => 5,
        RulePriority::DefaultAllow => 6,
    }
}

fn action_rank(action: &RuleAction) -> u8 {
    match action {
        RuleAction::Allow => 0,
        RuleAction::Ignore => 1,
        RuleAction::Block => 2,
    }
}

pub fn evaluate_domain_rule(domain: &str, rules: &[DomainRule]) -> RuleEvaluation {
    let normalized = domain.trim().to_ascii_lowercase();
    let mut best: Option<(u8, u8, u8, RuleAction, RulePriority, String, usize)> = None;

    for rule in rules.iter().filter(|r| r.enabled) {
        let exact_match = rule.domain.eq_ignore_ascii_case(&normalized);
        let wildcard_match = rule.domain.starts_with("*.") && normalized.ends_with(&rule.domain[1..]);

        if !exact_match && !wildcard_match {
            continue;
        }

        let rank = priority_rank(&rule.priority);
        let specificity = if exact_match { 2 } else { 1 };
        let action_rank = action_rank(&rule.action);
        let candidate = (
            rank,
            specificity,
            action_rank,
            rule.action.clone(),
            rule.priority.clone(),
            rule.domain.clone(),
            rule.domain.len(),
        );

        let should_replace = match &best {
            None => true,
            Some((existing_rank, existing_specificity, existing_action_rank, existing_action, existing_priority, existing_domain, existing_domain_len)) => {
                rank < *existing_rank
                    || (rank == *existing_rank && specificity > *existing_specificity)
                    || (rank == *existing_rank && specificity == *existing_specificity && action_rank < *existing_action_rank)
                    || (rank == *existing_rank && specificity == *existing_specificity && action_rank == *existing_action_rank && *existing_action == rule.action && rule.domain.len() < *existing_domain_len)
                    || (rank == *existing_rank && specificity == *existing_specificity && action_rank == *existing_action_rank && existing_domain == &rule.domain && existing_priority == &rule.priority)
            }
        };

        if should_replace {
            best = Some(candidate);
        }
    }

    match best {
        Some((_, _, _, action, priority, matched_rule, _)) => RuleEvaluation {
            action,
            priority,
            matched_rule: Some(matched_rule),
        },
        None => RuleEvaluation {
            action: RuleAction::Allow,
            priority: RulePriority::DefaultAllow,
            matched_rule: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_domain_rule, DomainRule, RuleAction, RulePriority};

    #[test]
    fn exact_match_blocks_ads() {
        let rules = vec![DomainRule {
            domain: "ads.example.com".to_string(),
            action: RuleAction::Block,
            priority: RulePriority::DownloadedLists,
            source: "test".to_string(),
            enabled: true,
        }];

        let result = evaluate_domain_rule("ads.example.com", &rules);
        assert_eq!(result.action, RuleAction::Block);
        assert_eq!(result.priority, RulePriority::DownloadedLists);
    }

    #[test]
    fn wildcard_match_blocks_subdomains() {
        let rules = vec![DomainRule {
            domain: "*.example.com".to_string(),
            action: RuleAction::Block,
            priority: RulePriority::DownloadedLists,
            source: "test".to_string(),
            enabled: true,
        }];

        let result = evaluate_domain_rule("tracker.example.com", &rules);
        assert_eq!(result.action, RuleAction::Block);
    }

    #[test]
    fn higher_priority_allowlist_beats_lower_priority_block() {
        let rules = vec![
            DomainRule {
                domain: "ads.example.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "test".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "ads.example.com".to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "user".to_string(),
                enabled: true,
            },
        ];

        let result = evaluate_domain_rule("ads.example.com", &rules);
        assert_eq!(result.action, RuleAction::Allow);
        assert_eq!(result.priority, RulePriority::UserAllowlist);
    }

    #[test]
    fn exact_match_takes_precedence_over_wildcard_match() {
        let rules = vec![
            DomainRule {
                domain: "*.example.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "wildcard".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "tracker.example.com".to_string(),
                action: RuleAction::Allow,
                priority: RulePriority::UserAllowlist,
                source: "exact".to_string(),
                enabled: true,
            },
        ];

        let result = evaluate_domain_rule("tracker.example.com", &rules);
        assert_eq!(result.action, RuleAction::Allow);
        assert_eq!(result.priority, RulePriority::UserAllowlist);
    }

    #[test]
    fn same_priority_exact_match_beats_wildcard_match() {
        let rules = vec![
            DomainRule {
                domain: "*.example.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "wildcard".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "tracker.example.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "exact".to_string(),
                enabled: true,
            },
        ];

        let result = evaluate_domain_rule("tracker.example.com", &rules);
        assert_eq!(result.action, RuleAction::Block);
        assert_eq!(result.matched_rule, Some("tracker.example.com".to_string()));
    }

    #[test]
    fn nested_subdomain_wildcards_match_practical_ad_hosts() {
        let rules = vec![
            DomainRule {
                domain: "*.googleads.g.doubleclick.net".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "nested-wildcard".to_string(),
                enabled: true,
            },
            DomainRule {
                domain: "*.ads.spotify.com".to_string(),
                action: RuleAction::Block,
                priority: RulePriority::DownloadedLists,
                source: "nested-wildcard".to_string(),
                enabled: true,
            },
        ];

        let first = evaluate_domain_rule("a.b.googleads.g.doubleclick.net", &rules);
        let second = evaluate_domain_rule("promo.ads.spotify.com", &rules);

        assert_eq!(first.action, RuleAction::Block);
        assert_eq!(second.action, RuleAction::Block);
    }
}
