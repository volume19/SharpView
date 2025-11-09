//! Arguments for Get-NetLocalGroup operations

use crate::args::NetworkCredential;
use crate::enums::MethodType;
use serde::{Deserialize, Serialize};

/// Arguments for Get-NetLocalGroup
///
/// Arguments for enumerating local groups on computers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetNetLocalGroup {
    /// Computer names to query
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Collection method (API or WinNT)
    #[serde(rename = "Method")]
    pub method: MethodType,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetNetLocalGroup {
    pub fn new() -> Self {
        // Default to current computer name (empty vec will be handled by implementation)
        Self {
            computer_name: vec![],
            method: MethodType::Api,
            credential: None,
        }
    }

    pub fn computer_name(mut self, names: Vec<String>) -> Self {
        self.computer_name = names;
        self
    }

    pub fn method(mut self, method: MethodType) -> Self {
        self.method = method;
        self
    }

    pub fn collection_method(self, method: MethodType) -> Self {
        self.method(method)
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetNetLocalGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_net_local_group_new() {
        let args = ArgsGetNetLocalGroup::new();
        assert_eq!(args.method, MethodType::Api);
    }

    #[test]
    fn test_args_get_net_local_group_builder() {
        let args = ArgsGetNetLocalGroup::new()
            .computer_name(vec!["DC01".to_string()])
            .method(MethodType::WinNT);

        assert_eq!(args.computer_name, vec!["DC01".to_string()]);
        assert_eq!(args.method, MethodType::WinNT);
    }
}
