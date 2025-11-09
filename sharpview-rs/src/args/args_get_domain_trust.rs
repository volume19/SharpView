//! Arguments for Get-DomainTrust operations

use crate::args::NetworkCredential;
use crate::enums::SearchScope;
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainTrust
///
/// Arguments for enumerating domain trusts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainTrust {
    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Use Windows API to enumerate
    #[serde(rename = "API")]
    pub api: bool,

    /// Use .NET methods to enumerate
    #[serde(rename = "NET")]
    pub net: bool,

    /// LDAP filter string
    #[serde(rename = "LDAPFilter", skip_serializing_if = "Option::is_none")]
    pub ldap_filter: Option<String>,

    /// Properties to load
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,

    /// The LDAP search base
    #[serde(rename = "SearchBase", skip_serializing_if = "Option::is_none")]
    pub search_base: Option<String>,

    /// The domain controller to query
    #[serde(rename = "Server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// The scope of the search
    #[serde(rename = "SearchScope")]
    pub search_scope: SearchScope,

    /// Result page size
    #[serde(rename = "ResultPageSize")]
    pub result_page_size: i32,

    /// Server time limit
    #[serde(rename = "ServerTimeLimit", skip_serializing_if = "Option::is_none")]
    pub server_time_limit: Option<i32>,

    /// Include tombstone objects
    #[serde(rename = "Tombstone")]
    pub tombstone: bool,

    /// Return only first result
    #[serde(rename = "FindOne")]
    pub find_one: bool,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetDomainTrust {
    pub fn new() -> Self {
        Self {
            domain: None,
            api: false,
            net: false,
            ldap_filter: None,
            properties: None,
            search_base: None,
            server: None,
            search_scope: SearchScope::Subtree,
            result_page_size: 200,
            server_time_limit: None,
            tombstone: false,
            find_one: false,
            credential: None,
        }
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn api(mut self, api: bool) -> Self {
        self.api = api;
        self
    }

    pub fn net(mut self, net: bool) -> Self {
        self.net = net;
        self
    }
}

impl Default for ArgsGetDomainTrust {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_trust_new() {
        let args = ArgsGetDomainTrust::new();
        assert_eq!(args.api, false);
        assert_eq!(args.net, false);
    }

    #[test]
    fn test_args_get_domain_trust_builder() {
        let args = ArgsGetDomainTrust::new()
            .domain("corp.local".to_string())
            .api(true);

        assert_eq!(args.domain, Some("corp.local".to_string()));
        assert_eq!(args.api, true);
    }
}
