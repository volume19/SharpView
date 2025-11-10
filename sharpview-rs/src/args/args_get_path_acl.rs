//! Arguments for Get-PathAcl operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-PathAcl
///
/// Gets the ACL for a specified file path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetPathAcl {
    /// File paths to query
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetPathAcl {
    pub fn new() -> Self {
        Self {
            path: None,
            credential: None,
        }
    }

    pub fn path(mut self, paths: Vec<String>) -> Self {
        self.path = Some(paths);
        self
    }

    pub fn full_name(self, paths: Vec<String>) -> Self {
        self.path(paths)
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetPathAcl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_path_acl_new() {
        let args = ArgsGetPathAcl::new();
        assert_eq!(args.path, None);
    }

    #[test]
    fn test_args_get_path_acl_builder() {
        let args = ArgsGetPathAcl::new()
            .path(vec!["C:\\Windows\\System32".to_string()]);

        assert_eq!(args.path, Some(vec!["C:\\Windows\\System32".to_string()]));
    }

    #[test]
    fn test_args_get_path_acl_alias() {
        let args = ArgsGetPathAcl::new()
            .full_name(vec!["C:\\Data".to_string()]);

        assert_eq!(args.path, Some(vec!["C:\\Data".to_string()]));
    }
}
