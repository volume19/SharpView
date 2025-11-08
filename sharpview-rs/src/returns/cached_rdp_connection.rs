//! Cached RDP connection information

use serde::{Deserialize, Serialize};

/// Cached RDP connection
///
/// Represents information about a cached Remote Desktop connection
/// stored on a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachedRdpConnection {
    /// The computer name where the connection is cached
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The user's SID
    #[serde(rename = "UserSID", skip_serializing_if = "Option::is_none")]
    pub user_sid: Option<String>,

    /// The target server that was connected to
    #[serde(rename = "TargetServer", skip_serializing_if = "Option::is_none")]
    pub target_server: Option<String>,

    /// The username hint (credential hint)
    #[serde(rename = "UsernameHint", skip_serializing_if = "Option::is_none")]
    pub username_hint: Option<String>,
}

impl CachedRdpConnection {
    /// Creates a new CachedRdpConnection
    pub fn new(
        computer_name: Option<String>,
        user_name: Option<String>,
        user_sid: Option<String>,
        target_server: Option<String>,
        username_hint: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            user_name,
            user_sid,
            target_server,
            username_hint,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cached_rdp_connection_new() {
        let conn = CachedRdpConnection::new(
            Some("WS01".to_string()),
            Some("john.doe".to_string()),
            Some("S-1-5-21-...-1234".to_string()),
            Some("DC01.domain.com".to_string()),
            Some("DOMAIN\\john.doe".to_string()),
        );
        assert_eq!(conn.computer_name, Some("WS01".to_string()));
        assert_eq!(conn.target_server, Some("DC01.domain.com".to_string()));
    }

    #[test]
    fn test_cached_rdp_connection_serde() {
        let conn = CachedRdpConnection::new(
            Some("WEB01".to_string()),
            Some("admin".to_string()),
            Some("S-1-5-21-...-500".to_string()),
            Some("192.168.1.10".to_string()),
            None,
        );

        let json = serde_json::to_string(&conn).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("UserName"));
        assert!(json.contains("TargetServer"));

        let deserialized: CachedRdpConnection = serde_json::from_str(&json).unwrap();
        assert_eq!(conn, deserialized);
    }
}
