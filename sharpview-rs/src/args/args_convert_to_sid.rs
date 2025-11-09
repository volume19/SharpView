//! Arguments for ConvertTo-SID operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for ConvertTo-SID
///
/// Converts a domain object name to a security identifier (SID).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsConvertToSid {
    /// Object name to convert
    #[serde(rename = "ObjectName", skip_serializing_if = "Option::is_none")]
    pub object_name: Option<Vec<String>>,

    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// The domain controller to query
    #[serde(rename = "Server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsConvertToSid {
    pub fn new() -> Self {
        Self {
            object_name: None,
            domain: None,
            server: None,
            credential: None,
        }
    }

    pub fn object_name(mut self, names: Vec<String>) -> Self {
        self.object_name = Some(names);
        self
    }

    pub fn name(self, names: Vec<String>) -> Self {
        self.object_name(names)
    }

    pub fn identity(self, names: Vec<String>) -> Self {
        self.object_name(names)
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn server(mut self, server: String) -> Self {
        self.server = Some(server);
        self
    }

    pub fn domain_controller(self, dc: String) -> Self {
        self.server(dc)
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsConvertToSid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_convert_to_sid_new() {
        let args = ArgsConvertToSid::new();
        assert_eq!(args.object_name, None);
    }

    #[test]
    fn test_args_convert_to_sid_builder() {
        let args = ArgsConvertToSid::new()
            .object_name(vec!["Administrator".to_string()])
            .domain("corp.local".to_string());

        assert_eq!(args.object_name, Some(vec!["Administrator".to_string()]));
        assert_eq!(args.domain, Some("corp.local".to_string()));
    }

    #[test]
    fn test_args_convert_to_sid_aliases() {
        let args1 = ArgsConvertToSid::new()
            .name(vec!["User1".to_string()]);
        assert_eq!(args1.object_name, Some(vec!["User1".to_string()]));

        let args2 = ArgsConvertToSid::new()
            .identity(vec!["User2".to_string()]);
        assert_eq!(args2.object_name, Some(vec!["User2".to_string()]));
    }
}
