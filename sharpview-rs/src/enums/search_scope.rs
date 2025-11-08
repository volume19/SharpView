//! LDAP search scope enumeration
//!
//! Maps to System.DirectoryServices.SearchScope from .NET

use serde::{Deserialize, Serialize};

/// LDAP search scope
///
/// Specifies the scope of an LDAP search operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SearchScope {
    /// Search only the base object
    Base,

    /// Search the base object and one level of child objects
    OneLevel,

    /// Search the base object and all descendant objects (recursive)
    Subtree,
}

impl Default for SearchScope {
    fn default() -> Self {
        Self::Subtree
    }
}

impl SearchScope {
    /// Returns LDAP scope integer value
    ///
    /// Base = 0, OneLevel = 1, Subtree = 2
    pub fn as_i32(self) -> i32 {
        match self {
            SearchScope::Base => 0,
            SearchScope::OneLevel => 1,
            SearchScope::Subtree => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_scope_values() {
        assert_eq!(SearchScope::Base.as_i32(), 0);
        assert_eq!(SearchScope::OneLevel.as_i32(), 1);
        assert_eq!(SearchScope::Subtree.as_i32(), 2);
    }

    #[test]
    fn test_search_scope_default() {
        assert_eq!(SearchScope::default(), SearchScope::Subtree);
    }

    #[test]
    fn test_search_scope_equality() {
        assert_eq!(SearchScope::Subtree, SearchScope::Subtree);
        assert_ne!(SearchScope::Base, SearchScope::OneLevel);
    }

    #[test]
    fn test_search_scope_serde() {
        let scope = SearchScope::Subtree;
        let json = serde_json::to_string(&scope).unwrap();
        assert_eq!(json, "\"Subtree\"");

        let deserialized: SearchScope = serde_json::from_str(&json).unwrap();
        assert_eq!(scope, deserialized);
    }

    #[test]
    fn test_search_scope_all_variants() {
        let test_cases = vec![
            (SearchScope::Base, "\"Base\"", 0),
            (SearchScope::OneLevel, "\"OneLevel\"", 1),
            (SearchScope::Subtree, "\"Subtree\"", 2),
        ];

        for (scope, expected_json, expected_i32) in test_cases {
            let json = serde_json::to_string(&scope).unwrap();
            assert_eq!(json, expected_json);
            assert_eq!(scope.as_i32(), expected_i32);

            let deserialized: SearchScope = serde_json::from_str(&json).unwrap();
            assert_eq!(scope, deserialized);
        }
    }
}
