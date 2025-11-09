//! Arguments for Get-DomainGroupMember operations

use crate::args::NetworkCredential;
use crate::enums::{SearchScope, SecurityMasks};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainGroupMember
///
/// Arguments for enumerating domain group members.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainGroupMember {
    /// Group identity (DN, SAM account name, etc.)
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    /// The target domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Recursively enumerate group members
    #[serde(rename = "Recurse")]
    pub recurse: bool,

    /// Use LDAP matching rule for recursive search
    #[serde(rename = "RecurseUsingMatchingRule")]
    pub recurse_using_matching_rule: bool,

    /// LDAP filter string
    #[serde(rename = "LDAPFilter", skip_serializing_if = "Option::is_none")]
    pub ldap_filter: Option<String>,

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

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetDomainGroupMember {
    pub fn new() -> Self {
        Self {
            identity: None,
            domain: None,
            recurse: false,
            recurse_using_matching_rule: false,
            ldap_filter: None,
            search_base: None,
            server: None,
            search_scope: SearchScope::Subtree,
            result_page_size: 200,
            server_time_limit: None,
            security_masks: None,
            tombstone: false,
            credential: None,
        }
    }

    pub fn identity(mut self, identity: Vec<String>) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn recurse(mut self, recurse: bool) -> Self {
        self.recurse = recurse;
        self
    }

    pub fn recurse_using_matching_rule(mut self, recurse: bool) -> Self {
        self.recurse_using_matching_rule = recurse;
        self
    }

    pub fn result_page_size(mut self, size: i32) -> Result<Self, String> {
        if size < 1 || size > 10000 {
            return Err("ResultPageSize must be between 1 and 10000".to_string());
        }
        self.result_page_size = size;
        Ok(self)
    }
}

impl Default for ArgsGetDomainGroupMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_group_member_new() {
        let args = ArgsGetDomainGroupMember::new();
        assert_eq!(args.recurse, false);
        assert_eq!(args.result_page_size, 200);
    }

    #[test]
    fn test_args_get_domain_group_member_builder() {
        let args = ArgsGetDomainGroupMember::new()
            .identity(vec!["Domain Admins".to_string()])
            .domain("corp.local".to_string())
            .recurse(true);

        assert_eq!(args.identity, Some(vec!["Domain Admins".to_string()]));
        assert_eq!(args.domain, Some("corp.local".to_string()));
        assert_eq!(args.recurse, true);
    }
}
