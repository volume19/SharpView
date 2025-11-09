//! Arguments for Get-DomainGPO operations

use crate::args::NetworkCredential;
use crate::enums::{SearchScope, SecurityMasks};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainGPO
///
/// Arguments for enumerating Group Policy Objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainGpo {
    /// GPO identity
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    /// Computer identity to find GPOs for
    #[serde(rename = "ComputerIdentity", skip_serializing_if = "Option::is_none")]
    pub computer_identity: Option<String>,

    /// User identity to find GPOs for
    #[serde(rename = "UserIdentity", skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,

    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

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

    /// Security masks
    #[serde(rename = "SecurityMasks", skip_serializing_if = "Option::is_none")]
    pub security_masks: Option<SecurityMasks>,

    /// Include tombstone objects
    #[serde(rename = "Tombstone")]
    pub tombstone: bool,

    /// Return only first result
    #[serde(rename = "FindOne")]
    pub find_one: bool,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,

    /// Return raw results
    #[serde(rename = "Raw")]
    pub raw: bool,
}

impl ArgsGetDomainGpo {
    pub fn new() -> Self {
        Self {
            identity: None,
            computer_identity: None,
            user_identity: None,
            domain: None,
            ldap_filter: None,
            properties: None,
            search_base: None,
            server: None,
            search_scope: SearchScope::Subtree,
            result_page_size: 200,
            server_time_limit: None,
            security_masks: None,
            tombstone: false,
            find_one: false,
            credential: None,
            raw: false,
        }
    }

    pub fn identity(mut self, identity: Vec<String>) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn computer_identity(mut self, identity: String) -> Self {
        self.computer_identity = Some(identity);
        self
    }

    pub fn user_identity(mut self, identity: String) -> Self {
        self.user_identity = Some(identity);
        self
    }
}

impl Default for ArgsGetDomainGpo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_gpo_new() {
        let args = ArgsGetDomainGpo::new();
        assert_eq!(args.identity, None);
        assert_eq!(args.result_page_size, 200);
    }

    #[test]
    fn test_args_get_domain_gpo_builder() {
        let args = ArgsGetDomainGpo::new()
            .computer_identity("DC01".to_string())
            .user_identity("admin".to_string());

        assert_eq!(args.computer_identity, Some("DC01".to_string()));
        assert_eq!(args.user_identity, Some("admin".to_string()));
    }
}
