//! Arguments for Get-WMIRegProxy operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-WMIRegProxy
///
/// Gets proxy settings via WMI registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetWmiRegProxy {
    /// Computer names to query (empty vec will default to current computer)
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetWmiRegProxy {
    pub fn new() -> Self {
        Self {
            computer_name: vec![],
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

impl Default for ArgsGetWmiRegProxy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_wmi_reg_proxy_new() {
        let args = ArgsGetWmiRegProxy::new();
        assert_eq!(args.computer_name, Vec::<String>::new());
    }

    #[test]
    fn test_args_get_wmi_reg_proxy_builder() {
        let args = ArgsGetWmiRegProxy::new()
            .computer_name(vec!["WS01".to_string()]);

        assert_eq!(args.computer_name, vec!["WS01".to_string()]);
    }
}
