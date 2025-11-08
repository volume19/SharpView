//! AD object class type enumeration

use serde::{Deserialize, Serialize};

/// Active Directory object class types
///
/// Represents the main object classes for filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClassType {
    /// User object class
    User,

    /// Group object class
    Group,

    /// Computer object class
    Computer,
}

impl ClassType {
    /// Returns the object class name for LDAP filters
    pub fn as_ldap_class(&self) -> &'static str {
        match self {
            ClassType::User => "user",
            ClassType::Group => "group",
            ClassType::Computer => "computer",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_type_as_ldap_class() {
        assert_eq!(ClassType::User.as_ldap_class(), "user");
        assert_eq!(ClassType::Group.as_ldap_class(), "group");
        assert_eq!(ClassType::Computer.as_ldap_class(), "computer");
    }

    #[test]
    fn test_class_type_serde() {
        let class_type = ClassType::User;
        let json = serde_json::to_string(&class_type).unwrap();
        assert_eq!(json, "\"User\"");

        let deserialized: ClassType = serde_json::from_str(&json).unwrap();
        assert_eq!(class_type, deserialized);
    }
}
