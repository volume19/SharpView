//! DFS share information

use serde::{Deserialize, Serialize};

/// DFS share
///
/// Represents information about a Distributed File System (DFS) share.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DfsShare {
    /// The share name
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The remote server name
    #[serde(rename = "RemoteServerName", skip_serializing_if = "Option::is_none")]
    pub remote_server_name: Option<String>,
}

impl DfsShare {
    /// Creates a new DfsShare
    pub fn new(name: Option<String>, remote_server_name: Option<String>) -> Self {
        Self {
            name,
            remote_server_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_share_new() {
        let share = DfsShare::new(
            Some("\\\\domain.com\\dfs\\shared".to_string()),
            Some("FILESERVER01".to_string()),
        );
        assert_eq!(share.name, Some("\\\\domain.com\\dfs\\shared".to_string()));
        assert_eq!(share.remote_server_name, Some("FILESERVER01".to_string()));
    }

    #[test]
    fn test_dfs_share_serde() {
        let share = DfsShare::new(
            Some("\\\\corp\\public".to_string()),
            Some("FS02".to_string()),
        );

        let json = serde_json::to_string(&share).unwrap();
        assert!(json.contains("Name"));
        assert!(json.contains("RemoteServerName"));

        let deserialized: DfsShare = serde_json::from_str(&json).unwrap();
        assert_eq!(share, deserialized);
    }
}
