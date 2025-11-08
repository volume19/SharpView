//! Registry-based mounted drive information

use serde::{Deserialize, Serialize};

/// Registry mounted drive
///
/// Represents information about a network drive mounted by a user,
/// retrieved from the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegMountedDrive {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The user's SID
    #[serde(rename = "UserSID", skip_serializing_if = "Option::is_none")]
    pub user_sid: Option<String>,

    /// The drive letter (e.g., "Z:")
    #[serde(rename = "DriveLetter", skip_serializing_if = "Option::is_none")]
    pub drive_letter: Option<String>,

    /// The provider name (e.g., "Microsoft Windows Network")
    #[serde(rename = "ProviderName", skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,

    /// The remote path (e.g., "\\\\server\\share")
    #[serde(rename = "RemotePath", skip_serializing_if = "Option::is_none")]
    pub remote_path: Option<String>,

    /// The username used for the drive connection
    #[serde(rename = "DriveUserName", skip_serializing_if = "Option::is_none")]
    pub drive_user_name: Option<String>,
}

impl RegMountedDrive {
    /// Creates a new RegMountedDrive
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        computer_name: Option<String>,
        user_name: Option<String>,
        user_sid: Option<String>,
        drive_letter: Option<String>,
        provider_name: Option<String>,
        remote_path: Option<String>,
        drive_user_name: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            user_name,
            user_sid,
            drive_letter,
            provider_name,
            remote_path,
            drive_user_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_mounted_drive_new() {
        let drive = RegMountedDrive::new(
            Some("WS01".to_string()),
            Some("john.doe".to_string()),
            Some("S-1-5-21-...-1234".to_string()),
            Some("Z:".to_string()),
            Some("Microsoft Windows Network".to_string()),
            Some("\\\\fileserver\\shared".to_string()),
            Some("domain\\john.doe".to_string()),
        );
        assert_eq!(drive.computer_name, Some("WS01".to_string()));
        assert_eq!(drive.drive_letter, Some("Z:".to_string()));
        assert_eq!(drive.remote_path, Some("\\\\fileserver\\shared".to_string()));
    }

    #[test]
    fn test_reg_mounted_drive_serde() {
        let drive = RegMountedDrive::new(
            Some("DC01".to_string()),
            Some("admin".to_string()),
            Some("S-1-5-21-...-500".to_string()),
            Some("H:".to_string()),
            None,
            Some("\\\\server\\home".to_string()),
            None,
        );

        let json = serde_json::to_string(&drive).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("DriveLetter"));
        assert!(json.contains("RemotePath"));

        let deserialized: RegMountedDrive = serde_json::from_str(&json).unwrap();
        assert_eq!(drive, deserialized);
    }
}
