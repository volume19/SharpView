//! Arguments for Test-AdminAccess operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Test-AdminAccess
///
/// Tests if the current user has administrative access to a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsTestAdminAccess {
    /// Computer names to test (default: localhost)
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsTestAdminAccess {
    pub fn new() -> Self {
        Self {
            computer_name: vec!["localhost".to_string()],
            credential: None,
        }
    }

    pub fn computer_name(mut self, names: Vec<String>) -> Self {
        self.computer_name = names;
        self
    }

    pub fn host_name(self, names: Vec<String>) -> Self {
        self.computer_name(names)
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsTestAdminAccess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_test_admin_access_new() {
        let args = ArgsTestAdminAccess::new();
        assert_eq!(args.computer_name, vec!["localhost".to_string()]);
    }

    #[test]
    fn test_args_test_admin_access_builder() {
        let args = ArgsTestAdminAccess::new()
            .computer_name(vec!["DC01".to_string()]);

        assert_eq!(args.computer_name, vec!["DC01".to_string()]);
    }
}
