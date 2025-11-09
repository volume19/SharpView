//! Arguments for Get-DomainGroup operations

use crate::args::{ArgsGetDomainObject, NetworkCredential};
use crate::enums::{GroupProperty, GroupScope, SearchScope, SecurityMasks};
use serde::{Deserialize, Serialize};

/// Arguments for Get-DomainGroup
///
/// Arguments for enumerating domain groups with various filters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgsGetDomainGroup {
    /// Group identity (DN, SAM account name, name, etc.)
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<String>>,

    /// Member identity to search for
    #[serde(rename = "MemberIdentity", skip_serializing_if = "Option::is_none")]
    pub member_identity: Option<Vec<String>>,

    /// Only groups with AdminCount=1
    #[serde(rename = "AdminCount")]
    pub admin_count: bool,

    /// Filter by group scope
    #[serde(rename = "GroupScope", skip_serializing_if = "Option::is_none")]
    pub group_scope: Option<GroupScope>,

    /// Filter by group property
    #[serde(rename = "GroupProperty", skip_serializing_if = "Option::is_none")]
    pub group_property: Option<GroupProperty>,

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
}

impl ArgsGetDomainGroup {
    pub fn new() -> Self {
        Self {
            identity: None,
            member_identity: None,
            admin_count: false,
            group_scope: None,
            group_property: None,
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

    pub fn from_domain_object(args: &ArgsGetDomainObject) -> Self {
        Self {
            identity: args.identity.clone(),
            member_identity: None,
            admin_count: false,
            group_scope: None,
            group_property: None,
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

    pub fn admin_count(mut self, admin_count: bool) -> Self {
        self.admin_count = admin_count;
        self
    }

    pub fn group_scope(mut self, scope: GroupScope) -> Self {
        self.group_scope = Some(scope);
        self
    }
}

impl Default for ArgsGetDomainGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_domain_group_new() {
        let args = ArgsGetDomainGroup::new();
        assert_eq!(args.admin_count, false);
        assert_eq!(args.result_page_size, 200);
    }

    #[test]
    fn test_args_get_domain_group_builder() {
        let args = ArgsGetDomainGroup::new()
            .admin_count(true)
            .group_scope(GroupScope::Global);

        assert_eq!(args.admin_count, true);
        assert_eq!(args.group_scope, Some(GroupScope::Global));
    }
}
