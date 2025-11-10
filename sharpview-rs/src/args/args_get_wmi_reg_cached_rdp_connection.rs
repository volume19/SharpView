//! Arguments for Get-WMIRegCachedRDPConnection operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-WMIRegCachedRDPConnection
///
/// Gets cached RDP connections via WMI registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetWmiRegCachedRdpConnection {
    /// Computer names to query (default: localhost)
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetWmiRegCachedRdpConnection {
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

impl Default for ArgsGetWmiRegCachedRdpConnection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_wmi_reg_cached_rdp_connection_new() {
        let args = ArgsGetWmiRegCachedRdpConnection::new();
        assert_eq!(args.computer_name, vec!["localhost".to_string()]);
    }

    #[test]
    fn test_args_get_wmi_reg_cached_rdp_connection_builder() {
        let args = ArgsGetWmiRegCachedRdpConnection::new()
            .computer_name(vec!["WS01".to_string()]);

        assert_eq!(args.computer_name, vec!["WS01".to_string()]);
    }
}
