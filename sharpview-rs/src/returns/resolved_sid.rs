//! Resolved SID information

use serde::{Deserialize, Serialize};

/// Resolved security identifier (SID) information
///
/// Represents a security identifier that has been resolved to include
/// additional information about the identity reference such as name,
/// domain, distinguished name, and object class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedSid {
    /// The name of the identity reference (e.g., username or group name)
    #[serde(rename = "IdentityReferenceName", skip_serializing_if = "Option::is_none")]
    pub identity_reference_name: Option<String>,

    /// The domain of the identity reference
    #[serde(rename = "IdentityReferenceDomain", skip_serializing_if = "Option::is_none")]
    pub identity_reference_domain: Option<String>,

    /// The distinguished name (DN) of the identity reference
    #[serde(rename = "IdentityReferenceDN", skip_serializing_if = "Option::is_none")]
    pub identity_reference_dn: Option<String>,

    /// The object class of the identity reference (e.g., "user", "group")
    #[serde(rename = "IdentityReferenceClass", skip_serializing_if = "Option::is_none")]
    pub identity_reference_class: Option<String>,
}

impl ResolvedSid {
    /// Creates a new ResolvedSid with all fields set to None
    pub fn new() -> Self {
        Self {
            identity_reference_name: None,
            identity_reference_domain: None,
            identity_reference_dn: None,
            identity_reference_class: None,
        }
    }

    /// Creates a new ResolvedSid with the specified values
    pub fn with_values(
        name: Option<String>,
        domain: Option<String>,
        dn: Option<String>,
        class: Option<String>,
    ) -> Self {
        Self {
            identity_reference_name: name,
            identity_reference_domain: domain,
            identity_reference_dn: dn,
            identity_reference_class: class,
        }
    }
}

impl Default for ResolvedSid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolved_sid_new() {
        let sid = ResolvedSid::new();
        assert_eq!(sid.identity_reference_name, None);
        assert_eq!(sid.identity_reference_domain, None);
        assert_eq!(sid.identity_reference_dn, None);
        assert_eq!(sid.identity_reference_class, None);
    }

    #[test]
    fn test_resolved_sid_with_values() {
        let sid = ResolvedSid::with_values(
            Some("Administrator".to_string()),
            Some("DOMAIN".to_string()),
            Some("CN=Administrator,CN=Users,DC=domain,DC=com".to_string()),
            Some("user".to_string()),
        );

        assert_eq!(sid.identity_reference_name, Some("Administrator".to_string()));
        assert_eq!(sid.identity_reference_domain, Some("DOMAIN".to_string()));
        assert_eq!(sid.identity_reference_dn, Some("CN=Administrator,CN=Users,DC=domain,DC=com".to_string()));
        assert_eq!(sid.identity_reference_class, Some("user".to_string()));
    }

    #[test]
    fn test_resolved_sid_serde() {
        let sid = ResolvedSid::with_values(
            Some("Administrator".to_string()),
            Some("DOMAIN".to_string()),
            None,
            Some("user".to_string()),
        );

        let json = serde_json::to_string(&sid).unwrap();
        let deserialized: ResolvedSid = serde_json::from_str(&json).unwrap();
        assert_eq!(sid, deserialized);
    }

    #[test]
    fn test_resolved_sid_serde_field_names() {
        let sid = ResolvedSid::with_values(
            Some("Admin".to_string()),
            None,
            None,
            None,
        );

        let json = serde_json::to_string(&sid).unwrap();
        assert!(json.contains("IdentityReferenceName"));
        assert!(!json.contains("IdentityReferenceDomain")); // Should be skipped since it's None
    }

    #[test]
    fn test_resolved_sid_default() {
        let sid = ResolvedSid::default();
        assert_eq!(sid, ResolvedSid::new());
    }
}
