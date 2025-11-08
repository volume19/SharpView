//! Registry-based logged on user information

use serde::{Deserialize, Serialize};

/// Registry logged on user
///
/// Represents information about a logged on user retrieved from the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegLoggedOnUser {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The user's domain
    #[serde(rename = "UserDomain", skip_serializing_if = "Option::is_none")]
    pub user_domain: Option<String>,

    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The user's SID
    #[serde(rename = "UserSID", skip_serializing_if = "Option::is_none")]
    pub user_sid: Option<String>,
}

impl RegLoggedOnUser {
    /// Creates a new RegLoggedOnUser
    pub fn new(
        computer_name: Option<String>,
        user_domain: Option<String>,
        user_name: Option<String>,
        user_sid: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            user_domain,
            user_name,
            user_sid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_logged_on_user_new() {
        let user = RegLoggedOnUser::new(
            Some("WS01".to_string()),
            Some("DOMAIN".to_string()),
            Some("john.doe".to_string()),
            Some("S-1-5-21-...-1234".to_string()),
        );
        assert_eq!(user.computer_name, Some("WS01".to_string()));
        assert_eq!(user.user_domain, Some("DOMAIN".to_string()));
        assert_eq!(user.user_name, Some("john.doe".to_string()));
    }

    #[test]
    fn test_reg_logged_on_user_serde() {
        let user = RegLoggedOnUser::new(
            Some("DC01".to_string()),
            Some("CORP".to_string()),
            Some("admin".to_string()),
            Some("S-1-5-21-...-500".to_string()),
        );

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("UserDomain"));
        assert!(json.contains("UserName"));
        assert!(json.contains("UserSID"));

        let deserialized: RegLoggedOnUser = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }
}
