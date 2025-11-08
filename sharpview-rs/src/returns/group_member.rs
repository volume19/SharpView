//! Group member information

use serde::{Deserialize, Serialize};

/// Group member
///
/// Represents information about a member of an Active Directory group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupMember {
    /// The group's domain
    #[serde(rename = "GroupDomain", skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,

    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The group's distinguished name
    #[serde(rename = "GroupDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub group_distinguished_name: Option<String>,

    /// The member's domain
    #[serde(rename = "MemberDomain", skip_serializing_if = "Option::is_none")]
    pub member_domain: Option<String>,

    /// The member name
    #[serde(rename = "MemberName", skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,

    /// The member's distinguished name
    #[serde(rename = "MemberDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub member_distinguished_name: Option<String>,

    /// The member's object class (e.g., "user", "group")
    #[serde(rename = "MemberObjectClass", skip_serializing_if = "Option::is_none")]
    pub member_object_class: Option<String>,

    /// The member's SID
    #[serde(rename = "MemberSID", skip_serializing_if = "Option::is_none")]
    pub member_sid: Option<String>,
}

impl GroupMember {
    /// Creates a new GroupMember
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        group_domain: Option<String>,
        group_name: Option<String>,
        group_distinguished_name: Option<String>,
        member_domain: Option<String>,
        member_name: Option<String>,
        member_distinguished_name: Option<String>,
        member_object_class: Option<String>,
        member_sid: Option<String>,
    ) -> Self {
        Self {
            group_domain,
            group_name,
            group_distinguished_name,
            member_domain,
            member_name,
            member_distinguished_name,
            member_object_class,
            member_sid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_member_new() {
        let member = GroupMember::new(
            Some("DOMAIN".to_string()),
            Some("Domain Admins".to_string()),
            Some("CN=Domain Admins,CN=Users,DC=domain,DC=com".to_string()),
            Some("DOMAIN".to_string()),
            Some("Administrator".to_string()),
            Some("CN=Administrator,CN=Users,DC=domain,DC=com".to_string()),
            Some("user".to_string()),
            Some("S-1-5-21-...-500".to_string()),
        );
        assert_eq!(member.group_name, Some("Domain Admins".to_string()));
        assert_eq!(member.member_name, Some("Administrator".to_string()));
        assert_eq!(member.member_object_class, Some("user".to_string()));
    }

    #[test]
    fn test_group_member_serde() {
        let member = GroupMember::new(
            Some("CORP".to_string()),
            Some("IT Admins".to_string()),
            None,
            Some("CORP".to_string()),
            Some("john.doe".to_string()),
            None,
            Some("user".to_string()),
            Some("S-1-5-21-...-1234".to_string()),
        );

        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("GroupDomain"));
        assert!(json.contains("GroupName"));
        assert!(json.contains("MemberName"));

        let deserialized: GroupMember = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);
    }
}
