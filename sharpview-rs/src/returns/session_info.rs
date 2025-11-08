//! Network session information

use serde::{Deserialize, Serialize};

/// Network session information
///
/// Represents information about an active network session on a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionInfo {
    /// The computer hosting the session
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The client computer name
    #[serde(rename = "CName", skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The username of the session
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// Time elapsed since the session started (in seconds)
    #[serde(rename = "Time")]
    pub time: u32,

    /// Time the session has been idle (in seconds)
    #[serde(rename = "IdleTime")]
    pub idle_time: u32,
}

impl SessionInfo {
    /// Creates a new SessionInfo
    pub fn new(
        computer_name: Option<String>,
        cname: Option<String>,
        user_name: Option<String>,
        time: u32,
        idle_time: u32,
    ) -> Self {
        Self {
            computer_name,
            cname,
            user_name,
            time,
            idle_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_info_new() {
        let session = SessionInfo::new(
            Some("DC01".to_string()),
            Some("WS01".to_string()),
            Some("administrator".to_string()),
            3600,
            300,
        );
        assert_eq!(session.computer_name, Some("DC01".to_string()));
        assert_eq!(session.cname, Some("WS01".to_string()));
        assert_eq!(session.user_name, Some("administrator".to_string()));
        assert_eq!(session.time, 3600);
        assert_eq!(session.idle_time, 300);
    }

    #[test]
    fn test_session_info_zero_times() {
        let session = SessionInfo::new(
            Some("WEB01".to_string()),
            None,
            Some("user".to_string()),
            0,
            0,
        );
        assert_eq!(session.time, 0);
        assert_eq!(session.idle_time, 0);
    }

    #[test]
    fn test_session_info_serde() {
        let session = SessionInfo::new(
            Some("DC01".to_string()),
            Some("WS01".to_string()),
            Some("admin".to_string()),
            1800,
            60,
        );
        let json = serde_json::to_string(&session).unwrap();

        assert!(json.contains("ComputerName"));
        assert!(json.contains("CName"));
        assert!(json.contains("UserName"));
        assert!(json.contains("Time"));
        assert!(json.contains("IdleTime"));

        let deserialized: SessionInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(session, deserialized);
    }
}
