//! Arguments for Get-NetRDPSession operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-NetRDPSession
///
/// Arguments for enumerating RDP sessions on computers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetNetRdpSession {
    /// Computer names to query (default: localhost)
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetNetRdpSession {
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

impl Default for ArgsGetNetRdpSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_net_rdp_session_new() {
        let args = ArgsGetNetRdpSession::new();
        assert_eq!(args.computer_name, vec!["localhost".to_string()]);
    }

    #[test]
    fn test_args_get_net_rdp_session_builder() {
        let args = ArgsGetNetRdpSession::new()
            .computer_name(vec!["RDS01".to_string(), "RDS02".to_string()]);

        assert_eq!(args.computer_name, vec!["RDS01".to_string(), "RDS02".to_string()]);
    }
}
