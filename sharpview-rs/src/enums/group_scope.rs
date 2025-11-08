//! Active Directory group scope enumeration

use serde::{Deserialize, Serialize};

/// Active Directory group scope
///
/// Defines the scope of a security or distribution group in Active Directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GroupScope {
    /// Domain local scope - can contain members from any domain but only grants
    /// permissions within the domain
    DomainLocal,

    /// Filter for groups that are NOT domain local
    NotDomainLocal,

    /// Global scope - can contain members from the same domain and can be granted
    /// permissions in any domain
    Global,

    /// Filter for groups that are NOT global
    NotGlobal,

    /// Universal scope - can contain members from any domain and can be granted
    /// permissions in any domain
    Universal,

    /// Filter for groups that are NOT universal
    NotUniversal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_scope_variants() {
        let scopes = vec![
            GroupScope::DomainLocal,
            GroupScope::NotDomainLocal,
            GroupScope::Global,
            GroupScope::NotGlobal,
            GroupScope::Universal,
            GroupScope::NotUniversal,
        ];
        assert_eq!(scopes.len(), 6);
    }

    #[test]
    fn test_group_scope_equality() {
        assert_eq!(GroupScope::Global, GroupScope::Global);
        assert_ne!(GroupScope::Global, GroupScope::Universal);
    }

    #[test]
    fn test_group_scope_serde() {
        let scope = GroupScope::Universal;
        let json = serde_json::to_string(&scope).unwrap();
        assert_eq!(json, "\"Universal\"");

        let deserialized: GroupScope = serde_json::from_str(&json).unwrap();
        assert_eq!(scope, deserialized);
    }

    #[test]
    fn test_group_scope_serde_all_variants() {
        let test_cases = vec![
            (GroupScope::DomainLocal, "\"DomainLocal\""),
            (GroupScope::Global, "\"Global\""),
            (GroupScope::Universal, "\"Universal\""),
        ];

        for (scope, expected_json) in test_cases {
            let json = serde_json::to_string(&scope).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: GroupScope = serde_json::from_str(&json).unwrap();
            assert_eq!(scope, deserialized);
        }
    }
}
