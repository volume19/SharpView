//! Group property filter enumeration

use serde::{Deserialize, Serialize};

/// Group property filters
///
/// Specifies group characteristics for filtering operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GroupProperty {
    /// Security-enabled groups
    Security,

    /// Distribution groups (non-security)
    Distribution,

    /// Groups created by the system
    CreatedBySystem,

    /// Groups not created by the system
    NotCreatedBySystem,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_property_variants() {
        let properties = vec![
            GroupProperty::Security,
            GroupProperty::Distribution,
            GroupProperty::CreatedBySystem,
            GroupProperty::NotCreatedBySystem,
        ];
        assert_eq!(properties.len(), 4);
    }

    #[test]
    fn test_group_property_serde() {
        let prop = GroupProperty::Security;
        let json = serde_json::to_string(&prop).unwrap();
        assert_eq!(json, "\"Security\"");

        let deserialized: GroupProperty = serde_json::from_str(&json).unwrap();
        assert_eq!(prop, deserialized);
    }
}
