//! Last logged on user information

use serde::{Deserialize, Serialize};

/// Last logged on user
///
/// Represents information about the last user to log on to a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastLoggedOnUser {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The username of the last logged on user
    #[serde(rename = "LastLoggedOn", skip_serializing_if = "Option::is_none")]
    pub last_logged_on: Option<String>,
}

impl LastLoggedOnUser {
    /// Creates a new LastLoggedOnUser
    pub fn new(computer_name: Option<String>, last_logged_on: Option<String>) -> Self {
        Self {
            computer_name,
            last_logged_on,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_logged_on_user_new() {
        let user = LastLoggedOnUser::new(
            Some("WS01".to_string()),
            Some("DOMAIN\\administrator".to_string()),
        );
        assert_eq!(user.computer_name, Some("WS01".to_string()));
        assert_eq!(user.last_logged_on, Some("DOMAIN\\administrator".to_string()));
    }

    #[test]
    fn test_last_logged_on_user_serde() {
        let user = LastLoggedOnUser::new(
            Some("DC01".to_string()),
            Some("admin".to_string()),
        );

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("LastLoggedOn"));

        let deserialized: LastLoggedOnUser = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }
}
