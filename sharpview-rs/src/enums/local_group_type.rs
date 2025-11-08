//! Local group type enumeration
//!
//! Well-known local security groups for privilege enumeration.

use serde::{Deserialize, Serialize};

/// Local security group types
///
/// Represents well-known local groups for administrative access checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LocalGroupType {
    /// Local Administrators group
    Administrators,

    /// Administrators group by SID (S-1-5-32-544)
    #[serde(rename = "S-1-5-32-544")]
    AdministratorsSid,

    /// Remote Desktop Users group (alias)
    #[serde(rename = "RDP")]
    Rdp,

    /// Remote Desktop Users group
    RemoteDesktopUsers,

    /// Remote Desktop Users group by SID (S-1-5-32-555)
    #[serde(rename = "S-1-5-32-555")]
    RemoteDesktopUsersSid,
}

impl LocalGroupType {
    /// Returns the group name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            LocalGroupType::Administrators => "Administrators",
            LocalGroupType::AdministratorsSid => "S-1-5-32-544",
            LocalGroupType::Rdp => "RDP",
            LocalGroupType::RemoteDesktopUsers => "Remote Desktop Users",
            LocalGroupType::RemoteDesktopUsersSid => "S-1-5-32-555",
        }
    }

    /// Returns the well-known SID for this group type
    pub fn sid(&self) -> Option<&'static str> {
        match self {
            LocalGroupType::Administrators | LocalGroupType::AdministratorsSid => {
                Some("S-1-5-32-544")
            }
            LocalGroupType::Rdp
            | LocalGroupType::RemoteDesktopUsers
            | LocalGroupType::RemoteDesktopUsersSid => Some("S-1-5-32-555"),
        }
    }

    /// Returns true if this is an administrators group variant
    pub fn is_administrators(&self) -> bool {
        matches!(
            self,
            LocalGroupType::Administrators | LocalGroupType::AdministratorsSid
        )
    }

    /// Returns true if this is a Remote Desktop Users group variant
    pub fn is_rdp_users(&self) -> bool {
        matches!(
            self,
            LocalGroupType::Rdp
                | LocalGroupType::RemoteDesktopUsers
                | LocalGroupType::RemoteDesktopUsersSid
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_group_type_as_str() {
        assert_eq!(LocalGroupType::Administrators.as_str(), "Administrators");
        assert_eq!(LocalGroupType::AdministratorsSid.as_str(), "S-1-5-32-544");
        assert_eq!(LocalGroupType::Rdp.as_str(), "RDP");
        assert_eq!(
            LocalGroupType::RemoteDesktopUsers.as_str(),
            "Remote Desktop Users"
        );
    }

    #[test]
    fn test_local_group_type_sid() {
        assert_eq!(
            LocalGroupType::Administrators.sid(),
            Some("S-1-5-32-544")
        );
        assert_eq!(
            LocalGroupType::AdministratorsSid.sid(),
            Some("S-1-5-32-544")
        );
        assert_eq!(LocalGroupType::Rdp.sid(), Some("S-1-5-32-555"));
        assert_eq!(
            LocalGroupType::RemoteDesktopUsers.sid(),
            Some("S-1-5-32-555")
        );
    }

    #[test]
    fn test_local_group_type_is_administrators() {
        assert!(LocalGroupType::Administrators.is_administrators());
        assert!(LocalGroupType::AdministratorsSid.is_administrators());
        assert!(!LocalGroupType::Rdp.is_administrators());
    }

    #[test]
    fn test_local_group_type_is_rdp_users() {
        assert!(LocalGroupType::Rdp.is_rdp_users());
        assert!(LocalGroupType::RemoteDesktopUsers.is_rdp_users());
        assert!(LocalGroupType::RemoteDesktopUsersSid.is_rdp_users());
        assert!(!LocalGroupType::Administrators.is_rdp_users());
    }

    #[test]
    fn test_local_group_type_serde() {
        let group = LocalGroupType::Administrators;
        let json = serde_json::to_string(&group).unwrap();
        assert_eq!(json, "\"Administrators\"");

        let deserialized: LocalGroupType = serde_json::from_str(&json).unwrap();
        assert_eq!(group, deserialized);
    }

    #[test]
    fn test_local_group_type_sid_serialization() {
        let group = LocalGroupType::AdministratorsSid;
        let json = serde_json::to_string(&group).unwrap();
        assert_eq!(json, "\"S-1-5-32-544\"");
    }
}
