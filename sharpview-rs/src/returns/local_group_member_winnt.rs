//! Local group member information (WinNT method)

use serde::{Deserialize, Serialize};

/// Local group member information from WinNT
///
/// Represents information about a member of a local group retrieved via WinNT provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalGroupMemberWinNt {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The account name of the member
    #[serde(rename = "AccountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,

    /// The SID of the member
    #[serde(rename = "SID", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    /// Whether the member is a group
    #[serde(rename = "IsGroup")]
    pub is_group: bool,

    /// Whether the member is from a domain
    #[serde(rename = "IsDomain")]
    pub is_domain: bool,
}

impl LocalGroupMemberWinNt {
    /// Creates a new LocalGroupMemberWinNt
    pub fn new(
        computer_name: Option<String>,
        group_name: Option<String>,
        account_name: Option<String>,
        sid: Option<String>,
        is_group: bool,
        is_domain: bool,
    ) -> Self {
        Self {
            computer_name,
            group_name,
            account_name,
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
    fn test_local_group_member_winnt_new() {
        let member = LocalGroupMemberWinNt::new(
            Some("DC01".to_string()),
            Some("Administrators".to_string()),
            Some("DOMAIN\\Administrator".to_string()),
            Some("S-1-5-21-...".to_string()),
            false,
            true,
        );
        assert_eq!(member.computer_name, Some("DC01".to_string()));
        assert_eq!(member.group_name, Some("Administrators".to_string()));
        assert!(!member.is_group);
        assert!(member.is_domain);
    }

    #[test]
    fn test_local_group_member_winnt_local_member() {
        let member = LocalGroupMemberWinNt::new(
            Some("WEB01".to_string()),
            Some("Users".to_string()),
            Some("LocalUser".to_string()),
            Some("S-1-5-21-...-1001".to_string()),
            false,
            false,
        );
        assert!(!member.is_domain);
    }

    #[test]
    fn test_local_group_member_winnt_serde() {
        let member = LocalGroupMemberWinNt::new(
            Some("DC01".to_string()),
            Some("Remote Desktop Users".to_string()),
            Some("john.doe".to_string()),
            Some("S-1-5-21-...-1234".to_string()),
            false,
            true,
        );

        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("GroupName"));
        assert!(json.contains("AccountName"));
        assert!(json.contains("IsGroup"));
        assert!(json.contains("IsDomain"));

        let deserialized: LocalGroupMemberWinNt = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);
    }
}
