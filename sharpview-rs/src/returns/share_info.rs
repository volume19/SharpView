//! Network share information

use serde::{Deserialize, Serialize};

/// Network share information
///
/// Represents information about a network share on a remote computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareInfo {
    /// The share name (e.g., "C$", "SYSVOL", "NETLOGON")
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The share type (numeric value)
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub share_type: Option<u32>,

    /// The share description/remark
    #[serde(rename = "Remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// The computer name hosting the share
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}

impl ShareInfo {
    /// Creates a new ShareInfo
    pub fn new(
        name: Option<String>,
        share_type: Option<u32>,
        remark: Option<String>,
        computer_name: Option<String>,
    ) -> Self {
        Self {
            name,
            share_type,
            remark,
            computer_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_share_info_new() {
        let share = ShareInfo::new(
            Some("SYSVOL".to_string()),
            Some(0),
            Some("Logon server share".to_string()),
            Some("DC01".to_string()),
        );
        assert_eq!(share.name, Some("SYSVOL".to_string()));
        assert_eq!(share.share_type, Some(0));
        assert_eq!(share.remark, Some("Logon server share".to_string()));
        assert_eq!(share.computer_name, Some("DC01".to_string()));
    }

    #[test]
    fn test_share_info_partial() {
        let share = ShareInfo::new(
            Some("C$".to_string()),
            Some(0x80000000),
            None,
            Some("WEB01".to_string()),
        );
        assert_eq!(share.name, Some("C$".to_string()));
        assert_eq!(share.remark, None);
    }

    #[test]
    fn test_share_info_serde() {
        let share = ShareInfo::new(
            Some("ADMIN$".to_string()),
            Some(0x80000000),
            Some("Remote Admin".to_string()),
            Some("DC01".to_string()),
        );
        let json = serde_json::to_string(&share).unwrap();

        assert!(json.contains("Name"));
        assert!(json.contains("Type"));
        assert!(json.contains("Remark"));
        assert!(json.contains("ComputerName"));

        let deserialized: ShareInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(share, deserialized);
    }
}
