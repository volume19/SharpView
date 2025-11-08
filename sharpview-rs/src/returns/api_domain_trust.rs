//! API-based domain trust information

use crate::enums::DsTrustType;
use crate::traits::DomainTrust;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// API domain trust
///
/// Represents domain trust information retrieved via Windows API
/// (DsEnumerateDomainTrusts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiDomainTrust {
    /// The source domain name
    #[serde(rename = "SourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,

    /// The target domain DNS name
    #[serde(rename = "TargetName", skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,

    /// The target domain NetBIOS name
    #[serde(rename = "TargetNetbiosName", skip_serializing_if = "Option::is_none")]
    pub target_netbios_name: Option<String>,

    /// Trust flags
    #[serde(rename = "Flags")]
    pub flags: u32,

    /// Parent index in trust enumeration
    #[serde(rename = "ParentIndex")]
    pub parent_index: u32,

    /// The type of trust (DS_DOMAIN_TRUST_TYPE)
    #[serde(rename = "TrustType")]
    pub trust_type: DsTrustType,

    /// Trust attributes bitfield
    #[serde(rename = "TrustAttributes")]
    pub trust_attributes: u32,

    /// The SID of the target domain
    #[serde(rename = "TargetSid", skip_serializing_if = "Option::is_none")]
    pub target_sid: Option<String>,

    /// The GUID of the target domain
    #[serde(rename = "TargetGuid")]
    pub target_guid: Uuid,
}

impl DomainTrust for ApiDomainTrust {
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
    fn test_api_domain_trust_creation() {
        let trust = ApiDomainTrust {
            source_name: Some("DOMAIN1".to_string()),
            target_name: Some("DOMAIN2.COM".to_string()),
            target_netbios_name: Some("DOMAIN2".to_string()),
            flags: 0x0001,
            parent_index: 0,
            trust_type: DsTrustType::InForest,
            trust_attributes: 0x0020,
            target_sid: Some("S-1-5-21-...".to_string()),
            target_guid: Uuid::nil(),
        };

        assert_eq!(trust.source_name, Some("DOMAIN1".to_string()));
        assert_eq!(trust.target_name, Some("DOMAIN2.COM".to_string()));
        assert_eq!(trust.trust_type, DsTrustType::InForest);
    }

    #[test]
    fn test_api_domain_trust_trait() {
        let mut trust = ApiDomainTrust {
            source_name: None,
            target_name: None,
            target_netbios_name: None,
            flags: 0,
            parent_index: 0,
            trust_type: DsTrustType::DirectOutbound,
            trust_attributes: 0,
            target_sid: None,
            target_guid: Uuid::nil(),
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
    fn test_api_domain_trust_serde() {
        let trust = ApiDomainTrust {
            source_name: Some("CORP".to_string()),
            target_name: Some("EXTERNAL.COM".to_string()),
            target_netbios_name: Some("EXTERNAL".to_string()),
            flags: 0x0005,
            parent_index: 1,
            trust_type: DsTrustType::DirectOutbound,
            trust_attributes: 0x0008,
            target_sid: Some("S-1-5-21-1234567890-1234567890-1234567890-500".to_string()),
            target_guid: Uuid::nil(),
        };

        let json = serde_json::to_string(&trust).unwrap();
        assert!(json.contains("SourceName"));
        assert!(json.contains("TargetName"));
        assert!(json.contains("TrustType"));

        let deserialized: ApiDomainTrust = serde_json::from_str(&json).unwrap();
        assert_eq!(trust, deserialized);
    }
}
