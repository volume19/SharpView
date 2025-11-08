//! Manager type enumeration

use serde::{Deserialize, Serialize};

/// Manager type for managed security groups
///
/// Specifies whether the manager is a group or a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ManagerType {
    /// Manager is a group object
    Group,

    /// Manager is a user object
    User,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_type_serde() {
        let manager = ManagerType::Group;
        let json = serde_json::to_string(&manager).unwrap();
        assert_eq!(json, "\"Group\"");

        let deserialized: ManagerType = serde_json::from_str(&json).unwrap();
        assert_eq!(manager, deserialized);
    }
}
