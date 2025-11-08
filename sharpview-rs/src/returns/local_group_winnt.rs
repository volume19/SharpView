//! Local group information (WinNT method)

use serde::{Deserialize, Serialize};

/// Local group information from WinNT
///
/// Represents information about a local group retrieved via WinNT provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalGroupWinNt {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The local group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The SID of the group
    #[serde(rename = "SID", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    /// Comment or description for the group
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl LocalGroupWinNt {
    /// Creates a new LocalGroupWinNt
    pub fn new(
        computer_name: Option<String>,
        group_name: Option<String>,
        sid: Option<String>,
        comment: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            group_name,
            sid,
            comment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_group_winnt_new() {
        let group = LocalGroupWinNt::new(
            Some("DC01".to_string()),
            Some("Administrators".to_string()),
            Some("S-1-5-32-544".to_string()),
            Some("Administrators have complete access".to_string()),
        );
        assert_eq!(group.computer_name, Some("DC01".to_string()));
        assert_eq!(group.group_name, Some("Administrators".to_string()));
        assert_eq!(group.sid, Some("S-1-5-32-544".to_string()));
    }

    #[test]
    fn test_local_group_winnt_serde() {
        let group = LocalGroupWinNt::new(
            Some("WEB01".to_string()),
            Some("Users".to_string()),
            Some("S-1-5-32-545".to_string()),
            None,
        );

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("GroupName"));
        assert!(json.contains("SID"));

        let deserialized: LocalGroupWinNt = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }
}
