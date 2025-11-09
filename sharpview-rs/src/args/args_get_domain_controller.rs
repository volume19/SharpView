//! Arguments for Get-DomainController operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainController
///
/// Arguments for enumerating domain controllers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetDomainController {
    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// The domain controller to query
    #[serde(rename = "Server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// Use LDAP to enumerate
    #[serde(rename = "LDAP")]
    pub ldap: bool,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetDomainController {
    pub fn new() -> Self {
        Self {
            domain: None,
            server: None,
            ldap: false,
            credential: None,
        }
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

    pub fn ldap(mut self, ldap: bool) -> Self {
        self.ldap = ldap;
        self
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetDomainController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_controller_new() {
        let args = ArgsGetDomainController::new();
        assert_eq!(args.ldap, false);
    }

    #[test]
    fn test_args_get_domain_controller_builder() {
        let args = ArgsGetDomainController::new()
            .domain("corp.local".to_string())
            .ldap(true);

        assert_eq!(args.domain, Some("corp.local".to_string()));
        assert_eq!(args.ldap, true);
    }

    #[test]
    fn test_args_get_domain_controller_serde() {
        let args = ArgsGetDomainController::new().ldap(true);
        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("LDAP"));

        let deserialized: ArgsGetDomainController = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
