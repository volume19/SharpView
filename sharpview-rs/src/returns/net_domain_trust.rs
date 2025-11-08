//! Network-based domain trust information

use crate::enums::{TrustDirection, TrustType};
use crate::traits::DomainTrust;
use serde::{Deserialize, Serialize};

/// Network domain trust
///
/// Represents domain trust information retrieved via .NET Framework's
/// System.DirectoryServices.ActiveDirectory namespace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetDomainTrust {
    /// The source domain name
    #[serde(rename = "SourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,

    /// The target domain name
    #[serde(rename = "TargetName", skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,

    /// The direction of the trust
    #[serde(rename = "TrustDirection")]
    pub trust_direction: TrustDirection,

    /// The type of trust
    #[serde(rename = "TrustType")]
    pub trust_type: TrustType,
}

impl DomainTrust for NetDomainTrust {
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
    fn test_net_domain_trust_creation() {
        let trust = NetDomainTrust {
            source_name: Some("CORP".to_string()),
            target_name: Some("PARTNER".to_string()),
            trust_direction: TrustDirection::Bidirectional,
            trust_type: TrustType::External,
        };

        assert_eq!(trust.source_name, Some("CORP".to_string()));
        assert_eq!(trust.target_name, Some("PARTNER".to_string()));
        assert_eq!(trust.trust_direction, TrustDirection::Bidirectional);
        assert_eq!(trust.trust_type, TrustType::External);
    }

    #[test]
    fn test_net_domain_trust_trait() {
        let mut trust = NetDomainTrust {
            source_name: None,
            target_name: None,
            trust_direction: TrustDirection::Outbound,
            trust_type: TrustType::ParentChild,
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
    fn test_net_domain_trust_all_directions() {
        let outbound = NetDomainTrust {
            source_name: Some("A".to_string()),
            target_name: Some("B".to_string()),
            trust_direction: TrustDirection::Outbound,
            trust_type: TrustType::External,
        };
        assert_eq!(outbound.trust_direction, TrustDirection::Outbound);

        let inbound = NetDomainTrust {
            source_name: Some("A".to_string()),
            target_name: Some("B".to_string()),
            trust_direction: TrustDirection::Inbound,
            trust_type: TrustType::External,
        };
        assert_eq!(inbound.trust_direction, TrustDirection::Inbound);

        let bidirectional = NetDomainTrust {
            source_name: Some("A".to_string()),
            target_name: Some("B".to_string()),
            trust_direction: TrustDirection::Bidirectional,
            trust_type: TrustType::External,
        };
        assert_eq!(bidirectional.trust_direction, TrustDirection::Bidirectional);
    }

    #[test]
    fn test_net_domain_trust_serde() {
        let trust = NetDomainTrust {
            source_name: Some("DOMAIN1".to_string()),
            target_name: Some("DOMAIN2".to_string()),
            trust_direction: TrustDirection::Inbound,
            trust_type: TrustType::Forest,
        };

        let json = serde_json::to_string(&trust).unwrap();
        assert!(json.contains("SourceName"));
        assert!(json.contains("TargetName"));
        assert!(json.contains("TrustDirection"));
        assert!(json.contains("TrustType"));

        let deserialized: NetDomainTrust = serde_json::from_str(&json).unwrap();
        assert_eq!(trust, deserialized);
    }
}
