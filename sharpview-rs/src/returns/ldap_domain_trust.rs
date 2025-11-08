//! LDAP-based domain trust information

use crate::enums::{TrustAttributeFlag, TrustDirection, TrustType};
use crate::traits::DomainTrust;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// LDAP domain trust
///
/// Represents domain trust information retrieved via LDAP queries
/// from Active Directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LdapDomainTrust {
    /// The source domain name
    #[serde(rename = "SourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,

    /// The target domain name
    #[serde(rename = "TargetName", skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,

    /// The type of trust
    #[serde(rename = "TrustType")]
    pub trust_type: TrustType,

    /// The direction of the trust
    #[serde(rename = "TrustDirection")]
    pub trust_direction: TrustDirection,

    /// Trust attribute flags
    #[serde(rename = "TrustAttributes")]
    pub trust_attributes: TrustAttributeFlag,

    /// When the trust was created
    #[serde(rename = "WhenCreated", skip_serializing_if = "Option::is_none")]
    pub when_created: Option<DateTime<Utc>>,

    /// When the trust was last changed
    #[serde(rename = "WhenChanged", skip_serializing_if = "Option::is_none")]
    pub when_changed: Option<DateTime<Utc>>,
}

impl DomainTrust for LdapDomainTrust {
    fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    fn target_name(&self) -> Option<&str> {
        self.target_name.as_deref()
    }

    fn set_source_name(&mut self, name: String) {
        self.source_name = Some(name);
    }

    fn set_target_name(&mut self, name: String) {
        self.target_name = Some(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldap_domain_trust_creation() {
        let trust = LdapDomainTrust {
            source_name: Some("CORP".to_string()),
            target_name: Some("PARTNER".to_string()),
            trust_type: TrustType::External,
            trust_direction: TrustDirection::Bidirectional,
            trust_attributes: TrustAttributeFlag::ForestTransitive,
            when_created: None,
            when_changed: None,
        };

        assert_eq!(trust.source_name, Some("CORP".to_string()));
        assert_eq!(trust.target_name, Some("PARTNER".to_string()));
        assert_eq!(trust.trust_type, TrustType::External);
        assert_eq!(trust.trust_direction, TrustDirection::Bidirectional);
    }

    #[test]
    fn test_ldap_domain_trust_trait() {
        let mut trust = LdapDomainTrust {
            source_name: None,
            target_name: None,
            trust_type: TrustType::ParentChild,
            trust_direction: TrustDirection::Outbound,
            trust_attributes: TrustAttributeFlag::WithinForest,
            when_created: None,
            when_changed: None,
        };

        // Test DomainTrust trait methods
        assert_eq!(trust.source_name(), None);
        trust.set_source_name("SOURCE".to_string());
        assert_eq!(trust.source_name(), Some("SOURCE"));

        assert_eq!(trust.target_name(), None);
        trust.set_target_name("TARGET".to_string());
        assert_eq!(trust.target_name(), Some("TARGET"));
    }

    #[test]
    fn test_ldap_domain_trust_with_timestamps() {
        let now = Utc::now();
        let trust = LdapDomainTrust {
            source_name: Some("DOMAIN1".to_string()),
            target_name: Some("DOMAIN2".to_string()),
            trust_type: TrustType::Forest,
            trust_direction: TrustDirection::Inbound,
            trust_attributes: TrustAttributeFlag::ForestTransitive,
            when_created: Some(now),
            when_changed: Some(now),
        };

        assert_eq!(trust.when_created, Some(now));
        assert_eq!(trust.when_changed, Some(now));
    }

    #[test]
    fn test_ldap_domain_trust_serde() {
        let trust = LdapDomainTrust {
            source_name: Some("CORP".to_string()),
            target_name: Some("EXTERNAL".to_string()),
            trust_type: TrustType::External,
            trust_direction: TrustDirection::Outbound,
            trust_attributes: TrustAttributeFlag::FilterSids,
            when_created: None,
            when_changed: None,
        };

        let json = serde_json::to_string(&trust).unwrap();
        assert!(json.contains("SourceName"));
        assert!(json.contains("TargetName"));
        assert!(json.contains("TrustType"));
        assert!(json.contains("TrustDirection"));

        let deserialized: LdapDomainTrust = serde_json::from_str(&json).unwrap();
        assert_eq!(trust, deserialized);
    }
}
