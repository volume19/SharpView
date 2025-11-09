//! Arguments for Get-DomainComputer operations

use crate::args::{ArgsGetDomainObject, ArgsGetDomainSearcher, NetworkCredential};
use crate::enums::{SearchScope, SecurityMasks, UacFlag};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainComputer
///
/// Arguments for enumerating domain computers with various filters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainComputer {
    /// Computer identity (SAM account name, DN, etc.)
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    /// Only computers with unconstrained delegation
    #[serde(rename = "Unconstrained")]
    pub unconstrained: bool,

    /// Only computers trusted to auth for delegation
    #[serde(rename = "TrustedToAuth")]
    pub trusted_to_auth: bool,

    /// Only computers that are printers
    #[serde(rename = "Printers")]
    pub printers: bool,

    /// Filter by SPN
    #[serde(rename = "SPN", skip_serializing_if = "Option::is_none")]
    pub spn: Option<String>,

    /// Filter by operating system
    #[serde(rename = "OperatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,

    /// Filter by service pack
    #[serde(rename = "ServicePack", skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,

    /// Filter by site name
    #[serde(rename = "SiteName", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,

    /// Ping computers to check if alive
    #[serde(rename = "Ping")]
    pub ping: bool,

    // Base search fields
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

    /// UAC filter
    #[serde(rename = "UACFilter", skip_serializing_if = "Option::is_none")]
    pub uac_filter: Option<UacFlag>,
}

impl ArgsGetDomainComputer {
    pub fn new() -> Self {
        Self {
            identity: None,
            unconstrained: false,
            trusted_to_auth: false,
            printers: false,
            spn: None,
            operating_system: None,
            service_pack: None,
            site_name: None,
            ping: false,
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
            uac_filter: None,
        }
    }

    pub fn from_searcher(args: &ArgsGetDomainSearcher) -> Self {
        Self {
            identity: None,
            unconstrained: false,
            trusted_to_auth: false,
            printers: false,
            spn: None,
            operating_system: None,
            service_pack: None,
            site_name: None,
            ping: false,
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
            uac_filter: None,
        }
    }

    pub fn from_domain_object(args: &ArgsGetDomainObject) -> Self {
        Self {
            identity: args.identity.clone(),
            unconstrained: false,
            trusted_to_auth: false,
            printers: false,
            spn: None,
            operating_system: None,
            service_pack: None,
            site_name: None,
            ping: false,
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
            uac_filter: args.uac_filter,
        }
    }

    pub fn unconstrained(mut self, unconstrained: bool) -> Self {
        self.unconstrained = unconstrained;
        self
    }

    pub fn operating_system(mut self, os: String) -> Self {
        self.operating_system = Some(os);
        self
    }
}

impl Default for ArgsGetDomainComputer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_computer_new() {
        let args = ArgsGetDomainComputer::new();
        assert_eq!(args.unconstrained, false);
        assert_eq!(args.ping, false);
    }

    #[test]
    fn test_args_get_domain_computer_builder() {
        let args = ArgsGetDomainComputer::new()
            .unconstrained(true)
            .operating_system("Windows Server 2019".to_string());

        assert_eq!(args.unconstrained, true);
        assert_eq!(args.operating_system, Some("Windows Server 2019".to_string()));
    }
}
