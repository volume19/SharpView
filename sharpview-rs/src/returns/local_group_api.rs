//! Local group information (API method)

use serde::{Deserialize, Serialize};

/// Local group information from API
///
/// Represents information about a local group retrieved via Windows API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalGroupApi {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The local group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Comment or description for the group
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl LocalGroupApi {
    /// Creates a new LocalGroupApi
    pub fn new(
        computer_name: Option<String>,
        group_name: Option<String>,
        comment: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            group_name,
            comment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_group_api_new() {
        let group = LocalGroupApi::new(
            Some("DC01".to_string()),
            Some("Administrators".to_string()),
            Some("Administrators have complete and unrestricted access".to_string()),
        );
        assert_eq!(group.computer_name, Some("DC01".to_string()));
        assert_eq!(group.group_name, Some("Administrators".to_string()));
        assert!(group.comment.is_some());
    }

    #[test]
    fn test_local_group_api_serde() {
        let group = LocalGroupApi::new(
            Some("WEB01".to_string()),
            Some("Remote Desktop Users".to_string()),
            None,
        );

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("GroupName"));

        let deserialized: LocalGroupApi = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }
}
