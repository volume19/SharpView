//! Arguments for Get-DomainObject operations

use crate::args::{ArgsGetDomainSearcher, NetworkCredential};
use crate::enums::{SearchScope, SecurityMasks, UacFlag};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainObject
///
/// Extends ArgsGetDomainSearcher with identity fields and additional options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainObject {
    /// Object identity (can be DN, SAM account name, name, etc.)
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    // Base search fields (from ArgsGetDomainSearcher)
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

    /// The scope of the search (default: Subtree)
    #[serde(rename = "SearchScope")]
    pub search_scope: SearchScope,

    /// Result page size (1-10000, default: 200)
    #[serde(rename = "ResultPageSize")]
    pub result_page_size: i32,

    /// Server time limit in seconds (1-10000)
    #[serde(rename = "ServerTimeLimit", skip_serializing_if = "Option::is_none")]
    pub server_time_limit: Option<i32>,

    /// Security masks for DACL/SACL retrieval
    #[serde(rename = "SecurityMasks", skip_serializing_if = "Option::is_none")]
    pub security_masks: Option<SecurityMasks>,

    /// Include tombstone (deleted) objects
    #[serde(rename = "Tombstone")]
    pub tombstone: bool,

    /// Return only the first result
    #[serde(rename = "FindOne")]
    pub find_one: bool,

    /// Network credential for authentication
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,

    /// Return raw LDAP results
    #[serde(rename = "Raw")]
    pub raw: bool,

    /// UAC filter flags
    #[serde(rename = "UACFilter", skip_serializing_if = "Option::is_none")]
    pub uac_filter: Option<UacFlag>,
}

impl ArgsGetDomainObject {
    /// Creates a new ArgsGetDomainObject with default values
    pub fn new() -> Self {
        Self {
            identity: None,
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

    /// Sets the identity
    pub fn identity(mut self, identity: Vec<String>) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Alias for identity (DistinguishedName)
    pub fn distinguished_name(self, dn: Vec<String>) -> Self {
        self.identity(dn)
    }

    /// Alias for identity (SamAccountName)
    pub fn sam_account_name(self, sam: Vec<String>) -> Self {
        self.identity(sam)
    }

    /// Alias for identity (Name)
    pub fn name(self, name: Vec<String>) -> Self {
        self.identity(name)
    }

    /// Sets the domain
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Sets the LDAP filter
    pub fn ldap_filter(mut self, filter: String) -> Self {
        self.ldap_filter = Some(filter);
        self
    }

    /// Alias for ldap_filter
    pub fn filter(self, filter: String) -> Self {
        self.ldap_filter(filter)
    }

    /// Sets the properties
    pub fn properties(mut self, properties: Vec<String>) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Sets the search base
    pub fn search_base(mut self, search_base: String) -> Self {
        self.search_base = Some(search_base);
        self
    }

    /// Alias for search_base
    pub fn ads_path(self, ads_path: String) -> Self {
        self.search_base(ads_path)
    }

    /// Sets the server
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

    /// Sets whether to return only one result
    pub fn find_one(mut self, find_one: bool) -> Self {
        self.find_one = find_one;
        self
    }

    /// Alias for find_one
    pub fn return_one(self, return_one: bool) -> Self {
        self.find_one(return_one)
    }

    /// Sets the credential
    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }

    /// Sets whether to return raw results
    pub fn raw(mut self, raw: bool) -> Self {
        self.raw = raw;
        self
    }

    /// Sets the UAC filter
    pub fn uac_filter(mut self, filter: UacFlag) -> Self {
        self.uac_filter = Some(filter);
        self
    }
}

impl Default for ArgsGetDomainObject {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_object_new() {
        let args = ArgsGetDomainObject::new();
        assert_eq!(args.identity, None);
        assert_eq!(args.domain, None);
        assert_eq!(args.search_scope, SearchScope::Subtree);
        assert_eq!(args.result_page_size, 200);
        assert_eq!(args.find_one, false);
        assert_eq!(args.raw, false);
    }

    #[test]
    fn test_args_get_domain_object_from_searcher() {
        let searcher = ArgsGetDomainSearcher::new()
            .domain("corp.local".to_string())
            .ldap_filter("(objectClass=user)".to_string());

        let object_args = ArgsGetDomainObject::from_searcher(&searcher);
        assert_eq!(object_args.domain, Some("corp.local".to_string()));
        assert_eq!(object_args.ldap_filter, Some("(objectClass=user)".to_string()));
    }

    #[test]
    fn test_args_get_domain_object_builder() {
        let args = ArgsGetDomainObject::new()
            .identity(vec!["CN=User1,DC=corp,DC=local".to_string()])
            .domain("corp.local".to_string())
            .find_one(true)
            .raw(true);

        assert_eq!(args.identity, Some(vec!["CN=User1,DC=corp,DC=local".to_string()]));
        assert_eq!(args.domain, Some("corp.local".to_string()));
        assert_eq!(args.find_one, true);
        assert_eq!(args.raw, true);
    }

    #[test]
    fn test_args_get_domain_object_aliases() {
        let args1 = ArgsGetDomainObject::new()
            .distinguished_name(vec!["CN=Test".to_string()]);
        assert_eq!(args1.identity, Some(vec!["CN=Test".to_string()]));

        let args2 = ArgsGetDomainObject::new()
            .sam_account_name(vec!["testuser".to_string()]);
        assert_eq!(args2.identity, Some(vec!["testuser".to_string()]));

        let args3 = ArgsGetDomainObject::new()
            .return_one(true);
        assert_eq!(args3.find_one, true);
    }

    #[test]
    fn test_args_get_domain_object_serde() {
        let args = ArgsGetDomainObject::new()
            .domain("test.local".to_string())
            .find_one(true);

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("Domain"));
        assert!(json.contains("FindOne"));

        let deserialized: ArgsGetDomainObject = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
