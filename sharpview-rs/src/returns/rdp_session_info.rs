//! RDP session information

use crate::enums::WtsConnectState;
use serde::{Deserialize, Serialize};

/// RDP session information
///
/// Represents information about a Remote Desktop Protocol (RDP) session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RdpSessionInfo {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The session name
    #[serde(rename = "SessionName", skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,

    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The session ID
    #[serde(rename = "ID")]
    pub id: i32,

    /// The connection state
    #[serde(rename = "State")]
    pub state: WtsConnectState,

    /// The source IP address
    #[serde(rename = "SourceIP", skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
}

impl RdpSessionInfo {
    /// Creates a new RdpSessionInfo
    pub fn new(
        computer_name: Option<String>,
        session_name: Option<String>,
        user_name: Option<String>,
        id: i32,
        state: WtsConnectState,
        source_ip: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            session_name,
            user_name,
            id,
            state,
            source_ip,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdp_session_info_new() {
        let session = RdpSessionInfo::new(
            Some("WS01".to_string()),
            Some("RDP-Tcp#0".to_string()),
            Some("john.doe".to_string()),
            2,
            WtsConnectState::Active,
            Some("192.168.1.100".to_string()),
        );
        assert_eq!(session.computer_name, Some("WS01".to_string()));
        assert_eq!(session.id, 2);
        assert_eq!(session.state, WtsConnectState::Active);
    }

    #[test]
    fn test_rdp_session_info_disconnected() {
        let session = RdpSessionInfo::new(
            Some("DC01".to_string()),
            Some("Console".to_string()),
            Some("admin".to_string()),
            1,
            WtsConnectState::Disconnected,
            None,
        );
        assert_eq!(session.state, WtsConnectState::Disconnected);
    }

    #[test]
    fn test_rdp_session_info_serde() {
        let session = RdpSessionInfo::new(
            Some("WEB01".to_string()),
            Some("RDP-Tcp#5".to_string()),
            Some("user".to_string()),
            10,
            WtsConnectState::Active,
            Some("10.0.0.50".to_string()),
        );

        let json = serde_json::to_string(&session).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("SessionName"));
        assert!(json.contains("ID"));
        assert!(json.contains("State"));

        let deserialized: RdpSessionInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(session, deserialized);
    }
}
