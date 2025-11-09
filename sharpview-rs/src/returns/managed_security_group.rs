//! Managed security group information

use crate::enums::ManagerType;
use serde::{Deserialize, Serialize};

/// Managed security group
///
/// Represents a security group with its manager information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSecurityGroup {
    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The group's distinguished name
    #[serde(rename = "GroupDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub group_distinguished_name: Option<String>,

    /// The manager's name
    #[serde(rename = "ManagerName", skip_serializing_if = "Option::is_none")]
    pub manager_name: Option<String>,

    /// The manager's distinguished name
    #[serde(rename = "ManagerDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub manager_distinguished_name: Option<String>,

    /// The manager type
    #[serde(rename = "ManagerType", skip_serializing_if = "Option::is_none")]
    pub manager_type: Option<ManagerType>,

    /// Whether the manager can write to the group
    #[serde(rename = "ManagerCanWrite", skip_serializing_if = "Option::is_none")]
    pub manager_can_write: Option<String>,
}

impl ManagedSecurityGroup {
    /// Creates a new ManagedSecurityGroup
    pub fn new(
        group_name: Option<String>,
        group_distinguished_name: Option<String>,
        manager_name: Option<String>,
        manager_distinguished_name: Option<String>,
        manager_type: Option<ManagerType>,
        manager_can_write: Option<String>,
    ) -> Self {
        Self {
            group_name,
            group_distinguished_name,
            manager_name,
            manager_distinguished_name,
            manager_type,
            manager_can_write,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_managed_security_group_new() {
        let group = ManagedSecurityGroup::new(
            Some("IT Admins".to_string()),
            Some("CN=IT Admins,OU=Groups,DC=domain,DC=com".to_string()),
            Some("John Doe".to_string()),
            Some("CN=John Doe,CN=Users,DC=domain,DC=com".to_string()),
            Some(ManagerType::User),
            Some("True".to_string()),
        );
        assert_eq!(group.group_name, Some("IT Admins".to_string()));
        assert_eq!(group.manager_type, Some(ManagerType::User));
        assert_eq!(group.manager_can_write, Some("True".to_string()));
    }

    #[test]
    fn test_managed_security_group_serde() {
        let group = ManagedSecurityGroup::new(
            Some("Developers".to_string()),
            None,
            Some("Dev Team Lead".to_string()),
            None,
            Some(ManagerType::Group),
            Some("False".to_string()),
        );

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("GroupName"));
        assert!(json.contains("ManagerName"));
        assert!(json.contains("ManagerType"));

        let deserialized: ManagedSecurityGroup = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }
}
