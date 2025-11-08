//! Foreign group member information

use serde::{Deserialize, Serialize};

/// Foreign group member
///
/// Represents a member of a group from a foreign (external) domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForeignGroupMember {
    /// The group's domain
    #[serde(rename = "GroupDomain", skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,

    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The group's distinguished name
    #[serde(rename = "GroupDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub group_distinguished_name: Option<String>,

    /// The member's domain (foreign domain)
    #[serde(rename = "MemberDomain", skip_serializing_if = "Option::is_none")]
    pub member_domain: Option<String>,

    /// The member name
    #[serde(rename = "MemberName", skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,

    /// The member's distinguished name
    #[serde(rename = "MemberDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub member_distinguished_name: Option<String>,
}

impl ForeignGroupMember {
    /// Creates a new ForeignGroupMember
    pub fn new(
        group_domain: Option<String>,
        group_name: Option<String>,
        group_distinguished_name: Option<String>,
        member_domain: Option<String>,
        member_name: Option<String>,
        member_distinguished_name: Option<String>,
    ) -> Self {
        Self {
            group_domain,
            group_name,
            group_distinguished_name,
            member_domain,
            member_name,
            member_distinguished_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_group_member_new() {
        let member = ForeignGroupMember::new(
            Some("DOMAIN1".to_string()),
            Some("Administrators".to_string()),
            Some("CN=Administrators,CN=Builtin,DC=domain1,DC=com".to_string()),
            Some("DOMAIN2".to_string()),
            Some("External User".to_string()),
            Some("CN=External User,CN=Users,DC=domain2,DC=com".to_string()),
        );
        assert_eq!(member.group_domain, Some("DOMAIN1".to_string()));
        assert_eq!(member.member_domain, Some("DOMAIN2".to_string()));
    }

    #[test]
    fn test_foreign_group_member_serde() {
        let member = ForeignGroupMember::new(
            Some("CORP".to_string()),
            Some("IT Group".to_string()),
            None,
            Some("PARTNER".to_string()),
            Some("partner.user".to_string()),
            None,
        );

        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("GroupDomain"));
        assert!(json.contains("MemberDomain"));

        let deserialized: ForeignGroupMember = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);
    }
}
