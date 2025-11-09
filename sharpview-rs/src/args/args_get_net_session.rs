//! Arguments for Get-NetSession operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-NetSession
///
/// Arguments for enumerating network sessions on computers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetNetSession {
    /// Computer names to query (default: localhost)
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetNetSession {
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

impl Default for ArgsGetNetSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_net_session_new() {
        let args = ArgsGetNetSession::new();
        assert_eq!(args.computer_name, vec!["localhost".to_string()]);
    }

    #[test]
    fn test_args_get_net_session_builder() {
        let args = ArgsGetNetSession::new()
            .computer_name(vec!["DC01".to_string(), "DC02".to_string()]);

        assert_eq!(args.computer_name, vec!["DC01".to_string(), "DC02".to_string()]);
    }

    #[test]
    fn test_args_get_net_session_serde() {
        let args = ArgsGetNetSession::new();
        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("ComputerName"));

        let deserialized: ArgsGetNetSession = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
