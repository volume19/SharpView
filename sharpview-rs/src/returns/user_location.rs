//! User location information

use serde::{Deserialize, Serialize};

/// User location information
///
/// Represents information about where a user is logged on in the network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLocation {
    /// The user's domain
    #[serde(rename = "UserDomain", skip_serializing_if = "Option::is_none")]
    pub user_domain: Option<String>,

    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The computer where the user is logged on
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The IP address of the computer
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// The session source (originating computer/IP)
    #[serde(rename = "SessionFrom", skip_serializing_if = "Option::is_none")]
    pub session_from: Option<String>,

    /// The resolved name of the session source
    #[serde(rename = "SessionFromName", skip_serializing_if = "Option::is_none")]
    pub session_from_name: Option<String>,

    /// Whether the user has local admin rights
    #[serde(rename = "LocalAdmin")]
    pub local_admin: bool,
}

impl UserLocation {
    /// Creates a new UserLocation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_domain: Option<String>,
        user_name: Option<String>,
        computer_name: Option<String>,
        ip_address: Option<String>,
        session_from: Option<String>,
        session_from_name: Option<String>,
        local_admin: bool,
    ) -> Self {
        Self {
            user_domain,
            user_name,
            computer_name,
            ip_address,
            session_from,
            session_from_name,
            local_admin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_location_new() {
        let location = UserLocation::new(
            Some("DOMAIN".to_string()),
            Some("administrator".to_string()),
            Some("DC01".to_string()),
            Some("192.168.1.10".to_string()),
            Some("192.168.1.100".to_string()),
            Some("WS01".to_string()),
            true,
        );
        assert_eq!(location.user_domain, Some("DOMAIN".to_string()));
        assert_eq!(location.user_name, Some("administrator".to_string()));
        assert_eq!(location.computer_name, Some("DC01".to_string()));
        assert_eq!(location.ip_address, Some("192.168.1.10".to_string()));
        assert_eq!(location.session_from, Some("192.168.1.100".to_string()));
        assert_eq!(location.session_from_name, Some("WS01".to_string()));
        assert!(location.local_admin);
    }

    #[test]
    fn test_user_location_no_admin() {
        let location = UserLocation::new(
            Some("DOMAIN".to_string()),
            Some("user".to_string()),
            Some("WEB01".to_string()),
            None,
            None,
            None,
            false,
        );
        assert!(!location.local_admin);
        assert_eq!(location.ip_address, None);
    }

    #[test]
    fn test_user_location_serde() {
        let location = UserLocation::new(
            Some("CORP".to_string()),
            Some("admin".to_string()),
            Some("DC01".to_string()),
            Some("10.0.0.1".to_string()),
            None,
            None,
            true,
        );
        let json = serde_json::to_string(&location).unwrap();

        assert!(json.contains("UserDomain"));
        assert!(json.contains("UserName"));
        assert!(json.contains("ComputerName"));
        assert!(json.contains("LocalAdmin"));

        let deserialized: UserLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(location, deserialized);
    }
}
