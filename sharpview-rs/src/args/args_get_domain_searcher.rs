//! Arguments for Get-DomainSearcher operations

use crate::args::NetworkCredential;
use crate::enums::{SearchScope, SecurityMasks};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainSearcher
///
/// Base arguments for LDAP domain searches.
/// Many other argument types build upon these fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainSearcher {
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

    /// Prefix for the search base
    #[serde(rename = "SearchBasePrefix", skip_serializing_if = "Option::is_none")]
    pub search_base_prefix: Option<String>,

    /// The domain controller to query
    #[serde(rename = "Server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// The scope of the search (default: Subtree)
    #[serde(rename = "SearchScope")]
    pub search_scope: SearchScope,

    /// Result page size (1-10000, default: 200)
    #[serde(rename = "ResultPageSize")]
    pub result_page_size: i32,

    /// Server time limit in seconds (1-10000, default: 120)
    #[serde(rename = "ServerTimeLimit", skip_serializing_if = "Option::is_none")]
    pub server_time_limit: Option<i32>,

    /// Security masks for DACL/SACL retrieval
    #[serde(rename = "SecurityMasks", skip_serializing_if = "Option::is_none")]
    pub security_masks: Option<SecurityMasks>,

    /// Include tombstone (deleted) objects
    #[serde(rename = "Tombstone")]
    pub tombstone: bool,

    /// Network credential for authentication
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetDomainSearcher {
    /// Creates a new ArgsGetDomainSearcher with default values
    pub fn new() -> Self {
        Self {
            domain: None,
            ldap_filter: None,
            properties: None,
            search_base: None,
            search_base_prefix: None,
            server: None,
            search_scope: SearchScope::Subtree,
            result_page_size: 200,
            server_time_limit: Some(120),
            security_masks: None,
            tombstone: false,
            credential: None,
        }
    }

    /// Sets the domain
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Sets the LDAP filter (also accepts "Filter" as alias)
    pub fn ldap_filter(mut self, filter: String) -> Self {
        self.ldap_filter = Some(filter);
        self
    }

    /// Alias for ldap_filter
    pub fn filter(self, filter: String) -> Self {
        self.ldap_filter(filter)
    }

    /// Sets the properties to retrieve
    pub fn properties(mut self, properties: Vec<String>) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Sets the search base (also accepts "ADSPath" as alias)
    pub fn search_base(mut self, search_base: String) -> Self {
        self.search_base = Some(search_base);
        self
    }

    /// Alias for search_base
    pub fn ads_path(self, ads_path: String) -> Self {
        self.search_base(ads_path)
    }

    /// Sets the search base prefix
    pub fn search_base_prefix(mut self, prefix: String) -> Self {
        self.search_base_prefix = Some(prefix);
        self
    }

    /// Sets the domain controller (also accepts "DomainController" as alias)
    pub fn server(mut self, server: String) -> Self {
        self.server = Some(server);
        self
    }

    /// Alias for server
    pub fn domain_controller(self, dc: String) -> Self {
        self.server(dc)
    }

    /// Sets the search scope
    pub fn search_scope(mut self, scope: SearchScope) -> Self {
        self.search_scope = scope;
        self
    }

    /// Sets the result page size (must be 1-10000)
    pub fn result_page_size(mut self, size: i32) -> Result<Self, String> {
        if size < 1 || size > 10000 {
            return Err("ResultPageSize must be between 1 and 10000".to_string());
        }
        self.result_page_size = size;
        Ok(self)
    }

    /// Sets the server time limit (must be 1-10000)
    pub fn server_time_limit(mut self, limit: i32) -> Result<Self, String> {
        if limit < 1 || limit > 10000 {
            return Err("ServerTimeLimit must be between 1 and 10000".to_string());
        }
        self.server_time_limit = Some(limit);
        Ok(self)
    }

    /// Sets the security masks
    pub fn security_masks(mut self, masks: SecurityMasks) -> Self {
        self.security_masks = Some(masks);
        self
    }

    /// Sets whether to include tombstone objects
    pub fn tombstone(mut self, tombstone: bool) -> Self {
        self.tombstone = tombstone;
        self
    }

    /// Sets the credential
    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetDomainSearcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_searcher_new() {
        let args = ArgsGetDomainSearcher::new();
        assert_eq!(args.domain, None);
        assert_eq!(args.search_scope, SearchScope::Subtree);
        assert_eq!(args.result_page_size, 200);
        assert_eq!(args.server_time_limit, Some(120));
        assert_eq!(args.tombstone, false);
    }

    #[test]
    fn test_args_get_domain_searcher_builder() {
        let args = ArgsGetDomainSearcher::new()
            .domain("corp.local".to_string())
            .ldap_filter("(objectClass=user)".to_string())
            .server("DC01.corp.local".to_string())
            .result_page_size(500)
            .unwrap();

        assert_eq!(args.domain, Some("corp.local".to_string()));
        assert_eq!(args.ldap_filter, Some("(objectClass=user)".to_string()));
        assert_eq!(args.server, Some("DC01.corp.local".to_string()));
        assert_eq!(args.result_page_size, 500);
    }

    #[test]
    fn test_args_get_domain_searcher_aliases() {
        let args1 = ArgsGetDomainSearcher::new()
            .filter("test".to_string());
        assert_eq!(args1.ldap_filter, Some("test".to_string()));

        let args2 = ArgsGetDomainSearcher::new()
            .ads_path("/DC=corp,DC=local".to_string());
        assert_eq!(args2.search_base, Some("/DC=corp,DC=local".to_string()));

        let args3 = ArgsGetDomainSearcher::new()
            .domain_controller("DC01".to_string());
        assert_eq!(args3.server, Some("DC01".to_string()));
    }

    #[test]
    fn test_args_get_domain_searcher_validation() {
        // Valid result_page_size
        let result = ArgsGetDomainSearcher::new().result_page_size(5000);
        assert!(result.is_ok());

        // Invalid result_page_size (too low)
        let result = ArgsGetDomainSearcher::new().result_page_size(0);
        assert!(result.is_err());

        // Invalid result_page_size (too high)
        let result = ArgsGetDomainSearcher::new().result_page_size(10001);
        assert!(result.is_err());

        // Valid server_time_limit
        let result = ArgsGetDomainSearcher::new().server_time_limit(300);
        assert!(result.is_ok());

        // Invalid server_time_limit
        let result = ArgsGetDomainSearcher::new().server_time_limit(20000);
        assert!(result.is_err());
    }

    #[test]
    fn test_args_get_domain_searcher_serde() {
        let args = ArgsGetDomainSearcher::new()
            .domain("test.local".to_string())
            .tombstone(true);

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("Domain"));
        assert!(json.contains("test.local"));

        let deserialized: ArgsGetDomainSearcher = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
