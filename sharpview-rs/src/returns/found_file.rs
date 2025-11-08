//! Found file information

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Found file
///
/// Represents information about a file found during a search operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoundFile {
    /// The file path
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// The owner of the file
    #[serde(rename = "Owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// Last access time
    #[serde(rename = "LastAccessTime")]
    pub last_access_time: DateTime<Utc>,

    /// Last write time
    #[serde(rename = "LastWriteTime")]
    pub last_write_time: DateTime<Utc>,

    /// Creation time
    #[serde(rename = "CreationTime")]
    pub creation_time: DateTime<Utc>,

    /// File length in bytes
    #[serde(rename = "Length")]
    pub length: i64,
}

impl FoundFile {
    /// Creates a new FoundFile
    pub fn new(
        path: Option<String>,
        owner: Option<String>,
        last_access_time: DateTime<Utc>,
        last_write_time: DateTime<Utc>,
        creation_time: DateTime<Utc>,
        length: i64,
    ) -> Self {
        Self {
            path,
            owner,
            last_access_time,
            last_write_time,
            creation_time,
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_found_file_new() {
        let now = Utc::now();
        let file = FoundFile::new(
            Some("C:\\Windows\\System32\\notepad.exe".to_string()),
            Some("BUILTIN\\Administrators".to_string()),
            now,
            now,
            now,
            193536,
        );
        assert_eq!(file.path, Some("C:\\Windows\\System32\\notepad.exe".to_string()));
        assert_eq!(file.length, 193536);
    }

    #[test]
    fn test_found_file_serde() {
        let now = Utc::now();
        let file = FoundFile::new(
            Some("C:\\temp\\test.txt".to_string()),
            Some("DOMAIN\\user".to_string()),
            now,
            now,
            now,
            1024,
        );

        let json = serde_json::to_string(&file).unwrap();
        assert!(json.contains("Path"));
        assert!(json.contains("Owner"));
        assert!(json.contains("LastAccessTime"));
        assert!(json.contains("Length"));

        let deserialized: FoundFile = serde_json::from_str(&json).unwrap();
        assert_eq!(file, deserialized);
    }
}
