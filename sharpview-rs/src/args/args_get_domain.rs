//! Arguments for Get-Domain operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-Domain
///
/// Simple arguments for domain enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetDomain {
    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Network credential for authentication
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetDomain {
    /// Creates a new ArgsGetDomain
    pub fn new() -> Self {
        Self {
            domain: None,
            credential: None,
        }
    }

    /// Sets the domain
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Sets the credential
    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetDomain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_new() {
        let args = ArgsGetDomain::new();
        assert_eq!(args.domain, None);
        assert_eq!(args.credential, None);
    }

    #[test]
    fn test_args_get_domain_builder() {
        let args = ArgsGetDomain::new()
            .domain("corp.local".to_string());

        assert_eq!(args.domain, Some("corp.local".to_string()));
    }

    #[test]
    fn test_args_get_domain_with_credential() {
        let cred = NetworkCredential::with_credentials(
            "admin".to_string(),
            "password".to_string(),
        );
        let args = ArgsGetDomain::new()
            .domain("test.local".to_string())
            .credential(cred.clone());

        assert_eq!(args.domain, Some("test.local".to_string()));
        assert_eq!(args.credential, Some(cred));
    }

    #[test]
    fn test_args_get_domain_serde() {
        let args = ArgsGetDomain::new()
            .domain("example.com".to_string());

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("Domain"));
        assert!(json.contains("example.com"));

        let deserialized: ArgsGetDomain = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
