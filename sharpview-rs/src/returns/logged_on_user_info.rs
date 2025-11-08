//! Logged on user information

use serde::{Deserialize, Serialize};

/// Logged on user information
///
/// Represents information about a user currently logged on to a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggedOnUserInfo {
    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The logon domain
    #[serde(rename = "LogonDomain", skip_serializing_if = "Option::is_none")]
    pub logon_domain: Option<String>,

    /// Authentication domains
    #[serde(rename = "AuthDomains", skip_serializing_if = "Option::is_none")]
    pub auth_domains: Option<String>,

    /// The logon server
    #[serde(rename = "LogonServer", skip_serializing_if = "Option::is_none")]
    pub logon_server: Option<String>,

    /// The computer name where the user is logged on
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}

impl LoggedOnUserInfo {
    /// Creates a new LoggedOnUserInfo
    pub fn new(
        user_name: Option<String>,
        logon_domain: Option<String>,
        auth_domains: Option<String>,
        logon_server: Option<String>,
        computer_name: Option<String>,
    ) -> Self {
        Self {
            user_name,
            logon_domain,
            auth_domains,
            logon_server,
            computer_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logged_on_user_info_new() {
        let info = LoggedOnUserInfo::new(
            Some("administrator".to_string()),
            Some("DOMAIN".to_string()),
            Some("DOMAIN1,DOMAIN2".to_string()),
            Some("DC01".to_string()),
            Some("WS01".to_string()),
        );
        assert_eq!(info.user_name, Some("administrator".to_string()));
        assert_eq!(info.logon_domain, Some("DOMAIN".to_string()));
        assert_eq!(info.computer_name, Some("WS01".to_string()));
    }

    #[test]
    fn test_logged_on_user_info_serde() {
        let info = LoggedOnUserInfo::new(
            Some("user".to_string()),
            Some("CORP".to_string()),
            None,
            Some("DC02".to_string()),
            Some("WEB01".to_string()),
        );

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("UserName"));
        assert!(json.contains("LogonDomain"));

        let deserialized: LoggedOnUserInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }
}
