//! Arguments for Get-DomainOU operations

use crate::args::{ArgsGetDomainObject, ArgsGetDomainSearcher, NetworkCredential};
use crate::enums::{SearchScope, SecurityMasks};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainOU
///
/// Arguments for enumerating Organizational Units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainOu {
    /// OU identity
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    /// GPLink/GUID filter
    #[serde(rename = "GPLink", skip_serializing_if = "Option::is_none")]
    pub gp_link: Option<String>,

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

impl ArgsGetDomainOu {
    pub fn new() -> Self {
        Self {
            identity: None,
            gp_link: None,
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

    pub fn from_searcher(args: &ArgsGetDomainSearcher) -> Self {
        Self {
            identity: None,
            gp_link: None,
            domain: args.domain.clone(),
            ldap_filter: args.ldap_filter.clone(),
            properties: args.properties.clone(),
            search_base: args.search_base.clone(),
            server: args.server.clone(),
            search_scope: args.search_scope,
            result_page_size: args.result_page_size,
            server_time_limit: args.server_time_limit,
            security_masks: args.security_masks,
            tombstone: args.tombstone,
            find_one: false,
            credential: args.credential.clone(),
            raw: false,
        }
    }

    pub fn from_domain_object(args: &ArgsGetDomainObject) -> Self {
        Self {
            identity: args.identity.clone(),
            gp_link: None,
            domain: args.domain.clone(),
            ldap_filter: args.ldap_filter.clone(),
            properties: args.properties.clone(),
            search_base: args.search_base.clone(),
            server: args.server.clone(),
            search_scope: args.search_scope,
            result_page_size: args.result_page_size,
            server_time_limit: args.server_time_limit,
            security_masks: args.security_masks,
            tombstone: args.tombstone,
            find_one: args.find_one,
            credential: args.credential.clone(),
            raw: args.raw,
        }
    }

    pub fn gp_link(mut self, link: String) -> Self {
        self.gp_link = Some(link);
        self
    }

    pub fn guid(self, guid: String) -> Self {
        self.gp_link(guid)
    }
}

impl Default for ArgsGetDomainOu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_ou_new() {
        let args = ArgsGetDomainOu::new();
        assert_eq!(args.gp_link, None);
    }

    #[test]
    fn test_args_get_domain_ou_builder() {
        let args = ArgsGetDomainOu::new()
            .gp_link("{12345678-1234-1234-1234-123456789012}".to_string());

        assert_eq!(args.gp_link, Some("{12345678-1234-1234-1234-123456789012}".to_string()));
    }
}
