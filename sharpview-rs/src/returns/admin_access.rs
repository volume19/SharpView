//! Admin access check result

use serde::{Deserialize, Serialize};

/// Admin access check result
///
/// Represents the result of checking if the current user has
/// administrative access to a remote computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAccess {
    /// The name of the computer that was checked
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// Whether the user has administrative access
    #[serde(rename = "IsAdmin")]
    pub is_admin: bool,
}

impl AdminAccess {
    /// Creates a new AdminAccess result
    pub fn new(computer_name: Option<String>, is_admin: bool) -> Self {
        Self {
            computer_name,
            is_admin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_access_new() {
        let access = AdminAccess::new(Some("DC01".to_string()), true);
        assert_eq!(access.computer_name, Some("DC01".to_string()));
        assert!(access.is_admin);
    }

    #[test]
    fn test_admin_access_no_admin() {
        let access = AdminAccess::new(Some("WEB01".to_string()), false);
        assert_eq!(access.computer_name, Some("WEB01".to_string()));
        assert!(!access.is_admin);
    }

    #[test]
    fn test_admin_access_serde() {
        let access = AdminAccess::new(Some("DC01".to_string()), true);
        let json = serde_json::to_string(&access).unwrap();

        assert!(json.contains("ComputerName"));
        assert!(json.contains("IsAdmin"));

        let deserialized: AdminAccess = serde_json::from_str(&json).unwrap();
        assert_eq!(access, deserialized);
    }

    #[test]
    fn test_admin_access_serde_field_names() {
        let access = AdminAccess::new(None, false);
        let json = serde_json::to_string(&access).unwrap();

        // ComputerName should be skipped when None
        assert!(!json.contains("ComputerName"));
        assert!(json.contains("IsAdmin"));
    }
}
