//! Local group member information (API method)

use serde::{Deserialize, Serialize};

/// Local group member information from API
///
/// Represents information about a member of a local group retrieved via Windows API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalGroupMemberApi {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The member name
    #[serde(rename = "MemberName", skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,

    /// The SID of the member
    #[serde(rename = "SID", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    /// Whether the member is a group
    #[serde(rename = "IsGroup")]
    pub is_group: bool,

    /// Whether the member is from a domain (as a string in C# version)
    #[serde(rename = "IsDomain", skip_serializing_if = "Option::is_none")]
    pub is_domain: Option<String>,
}

impl LocalGroupMemberApi {
    /// Creates a new LocalGroupMemberApi
    pub fn new(
        computer_name: Option<String>,
        group_name: Option<String>,
        member_name: Option<String>,
        sid: Option<String>,
        is_group: bool,
        is_domain: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            group_name,
            member_name,
            sid,
            is_group,
            is_domain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_group_member_api_new() {
        let member = LocalGroupMemberApi::new(
            Some("DC01".to_string()),
            Some("Administrators".to_string()),
            Some("DOMAIN\\Administrator".to_string()),
            Some("S-1-5-21-...".to_string()),
            false,
            Some("DOMAIN".to_string()),
        );
        assert_eq!(member.computer_name, Some("DC01".to_string()));
        assert_eq!(member.group_name, Some("Administrators".to_string()));
        assert!(!member.is_group);
        assert_eq!(member.is_domain, Some("DOMAIN".to_string()));
    }

    #[test]
    fn test_local_group_member_api_group_member() {
        let member = LocalGroupMemberApi::new(
            Some("WEB01".to_string()),
            Some("Administrators".to_string()),
            Some("Domain Admins".to_string()),
            Some("S-1-5-21-...-512".to_string()),
            true,
            Some("DOMAIN".to_string()),
        );
        assert!(member.is_group);
    }

    #[test]
    fn test_local_group_member_api_serde() {
        let member = LocalGroupMemberApi::new(
            Some("DC01".to_string()),
            Some("Remote Desktop Users".to_string()),
            Some("john.doe".to_string()),
            None,
            false,
            None,
        );

        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("GroupName"));
        assert!(json.contains("MemberName"));
        assert!(json.contains("IsGroup"));

        let deserialized: LocalGroupMemberApi = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);
    }
}
