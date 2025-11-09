//! Arguments for ConvertFrom-SID operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for ConvertFrom-SID
///
/// Converts a security identifier (SID) to a domain object name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsConvertFromSid {
    /// Object SID to convert
    #[serde(rename = "ObjectSID", skip_serializing_if = "Option::is_none")]
    pub object_sid: Option<Vec<String>>,

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

impl ArgsConvertFromSid {
    pub fn new() -> Self {
        Self {
            object_sid: None,
            domain: None,
            server: None,
            credential: None,
        }
    }

    pub fn object_sid(mut self, sids: Vec<String>) -> Self {
        self.object_sid = Some(sids);
        self
    }

    pub fn sid(self, sids: Vec<String>) -> Self {
        self.object_sid(sids)
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

impl Default for ArgsConvertFromSid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_convert_from_sid_new() {
        let args = ArgsConvertFromSid::new();
        assert_eq!(args.object_sid, None);
    }

    #[test]
    fn test_args_convert_from_sid_builder() {
        let args = ArgsConvertFromSid::new()
            .object_sid(vec!["S-1-5-21-123456789-123456789-123456789-500".to_string()])
            .domain("corp.local".to_string());

        assert_eq!(args.object_sid, Some(vec!["S-1-5-21-123456789-123456789-123456789-500".to_string()]));
        assert_eq!(args.domain, Some("corp.local".to_string()));
    }

    #[test]
    fn test_args_convert_from_sid_alias() {
        let args = ArgsConvertFromSid::new()
            .sid(vec!["S-1-5-32-544".to_string()]);
        assert_eq!(args.object_sid, Some(vec!["S-1-5-32-544".to_string()]));
    }
}
