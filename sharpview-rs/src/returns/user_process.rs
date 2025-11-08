//! User process information

use serde::{Deserialize, Serialize};

/// User process information
///
/// Represents information about a process running under a user's context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProcess {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The process name
    #[serde(rename = "ProcessName", skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,

    /// The process ID (as a string in C# version)
    #[serde(rename = "ProcessID", skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,

    /// The domain of the user running the process
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// The username running the process
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl UserProcess {
    /// Creates a new UserProcess
    pub fn new(
        computer_name: Option<String>,
        process_name: Option<String>,
        process_id: Option<String>,
        domain: Option<String>,
        user: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            process_name,
            process_id,
            domain,
            user,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_process_new() {
        let proc = UserProcess::new(
            Some("WS01".to_string()),
            Some("notepad.exe".to_string()),
            Some("1234".to_string()),
            Some("DOMAIN".to_string()),
            Some("john.doe".to_string()),
        );
        assert_eq!(proc.computer_name, Some("WS01".to_string()));
        assert_eq!(proc.process_name, Some("notepad.exe".to_string()));
        assert_eq!(proc.process_id, Some("1234".to_string()));
    }

    #[test]
    fn test_user_process_serde() {
        let proc = UserProcess::new(
            Some("DC01".to_string()),
            Some("powershell.exe".to_string()),
            Some("5678".to_string()),
            Some("CORP".to_string()),
            Some("admin".to_string()),
        );

        let json = serde_json::to_string(&proc).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("ProcessName"));
        assert!(json.contains("ProcessID"));
        assert!(json.contains("Domain"));
        assert!(json.contains("User"));

        let deserialized: UserProcess = serde_json::from_str(&json).unwrap();
        assert_eq!(proc, deserialized);
    }
}
