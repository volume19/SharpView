//! Arguments for Resolve-IPAddress operations

use serde::{Deserialize, Serialize};

/// Arguments for Resolve-IPAddress
///
/// Resolves hostnames to IP addresses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsResolveIpAddress {
    /// Computer names to resolve (empty vec will default to current computer)
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,
}

impl ArgsResolveIpAddress {
    pub fn new() -> Self {
        Self {
            computer_name: vec![],
        }
    }

    pub fn computer_name(mut self, names: Vec<String>) -> Self {
        self.computer_name = names;
        self
    }

    pub fn host_name(self, names: Vec<String>) -> Self {
        self.computer_name(names)
    }
}

impl Default for ArgsResolveIpAddress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_resolve_ip_address_new() {
        let args = ArgsResolveIpAddress::new();
        assert_eq!(args.computer_name, Vec::<String>::new());
    }

    #[test]
    fn test_args_resolve_ip_address_builder() {
        let args = ArgsResolveIpAddress::new()
            .computer_name(vec!["DC01.corp.local".to_string()]);

        assert_eq!(args.computer_name, vec!["DC01.corp.local".to_string()]);
    }

    #[test]
    fn test_args_resolve_ip_address_alias() {
        let args = ArgsResolveIpAddress::new()
            .host_name(vec!["www.example.com".to_string()]);

        assert_eq!(args.computer_name, vec!["www.example.com".to_string()]);
    }
}
