//! Arguments for Get-DomainUser operations

use crate::args::{ArgsGetDomainSearcher, NetworkCredential};
use crate::enums::{SearchScope, SecurityMasks, UacFlag};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainUser
///
/// Arguments for enumerating domain users with various filters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainUser {
    /// User identity (can be DN, SAM account name, name, etc.)
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    /// Only users with SPN set
    #[serde(rename = "SPN")]
    pub spn: bool,

    /// Only users with AdminCount=1
    #[serde(rename = "AdminCount")]
    pub admin_count: bool,

    /// Only users allowed to delegate
    #[serde(rename = "AllowDelegation")]
    pub allow_delegation: bool,

    /// Only users disallowed delegation
    #[serde(rename = "DisallowDelegation")]
    pub disallow_delegation: bool,

    /// Only users trusted to auth for delegation
    #[serde(rename = "TrustedToAuth")]
    pub trusted_to_auth: bool,

    /// Only users that don't require Kerberos preauthentication
    #[serde(rename = "PreauthNotRequired")]
    pub preauth_not_required: bool,

    // Base search fields
    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// LDAP filter string
    #[serde(rename = "LDAPFilter", skip_serializing_if = "Option::is_none")]
    pub ldap_filter: Option<String>,

    /// Properties to load for each object
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

    /// Result page size (1-10000)
    #[serde(rename = "ResultPageSize")]
    pub result_page_size: i32,

    /// Server time limit in seconds
    #[serde(rename = "ServerTimeLimit", skip_serializing_if = "Option::is_none")]
    pub server_time_limit: Option<i32>,

    /// Security masks for DACL/SACL retrieval
    #[serde(rename = "SecurityMasks", skip_serializing_if = "Option::is_none")]
    pub security_masks: Option<SecurityMasks>,

    /// Include tombstone objects
    #[serde(rename = "Tombstone")]
    pub tombstone: bool,

    /// Return only the first result
    #[serde(rename = "FindOne")]
    pub find_one: bool,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,

    /// Return raw LDAP results
    #[serde(rename = "Raw")]
    pub raw: bool,

    /// UAC filter flags
    #[serde(rename = "UACFilter", skip_serializing_if = "Option::is_none")]
    pub uac_filter: Option<UacFlag>,
}

impl ArgsGetDomainUser {
    /// Creates a new ArgsGetDomainUser with default values
    pub fn new() -> Self {
        Self {
            identity: None,
            spn: false,
            admin_count: false,
            allow_delegation: false,
            disallow_delegation: false,
            trusted_to_auth: false,
            preauth_not_required: false,
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

    /// Creates from ArgsGetDomainSearcher
    pub fn from_searcher(args: &ArgsGetDomainSearcher) -> Self {
        Self {
            identity: None,
            spn: false,
            admin_count: false,
            allow_delegation: false,
            disallow_delegation: false,
            trusted_to_auth: false,
            preauth_not_required: false,
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

    /// Builder methods (abbreviated for brevity - similar to ArgsGetDomainObject)
    pub fn identity(mut self, identity: Vec<String>) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn spn(mut self, spn: bool) -> Self {
        self.spn = spn;
        self
    }

    pub fn admin_count(mut self, admin_count: bool) -> Self {
        self.admin_count = admin_count;
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn server(mut self, server: String) -> Self {
        self.server = Some(server);
        self
    }
}

impl Default for ArgsGetDomainUser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_user_new() {
        let args = ArgsGetDomainUser::new();
        assert_eq!(args.spn, false);
        assert_eq!(args.admin_count, false);
        assert_eq!(args.result_page_size, 200);
    }

    #[test]
    fn test_args_get_domain_user_builder() {
        let args = ArgsGetDomainUser::new()
            .domain("corp.local".to_string())
            .spn(true)
            .admin_count(true);

        assert_eq!(args.domain, Some("corp.local".to_string()));
        assert_eq!(args.spn, true);
        assert_eq!(args.admin_count, true);
    }

    #[test]
    fn test_args_get_domain_user_serde() {
        let args = ArgsGetDomainUser::new()
            .spn(true);

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("SPN"));

        let deserialized: ArgsGetDomainUser = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
