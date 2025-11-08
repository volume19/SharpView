//! Active Directory group type flags
//!
//! Maps to groupType attribute values in Active Directory.

use serde::{Deserialize, Serialize};

/// Group type flags
///
/// Bitflags representing group characteristics in Active Directory.
/// These map to the groupType attribute and ADS_GROUP_TYPE_ENUM COM values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum GroupTypeFlag {
    /// Group created by the system
    CreatedBySystem = 0x00000001,

    /// Global scope group (ADS_GROUP_TYPE_GLOBAL_GROUP)
    GlobalScope = 0x00000002,

    /// Domain local scope group (ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP)
    DomainLocalScope = 0x00000004,

    /// Universal scope group (ADS_GROUP_TYPE_UNIVERSAL_GROUP)
    UniversalScope = 0x00000008,

    /// Application basic group
    AppBasic = 0x00000010,

    /// Application query group
    AppQuery = 0x00000020,

    /// Security-enabled group (ADS_GROUP_TYPE_SECURITY_ENABLED)
    Security = 0x80000000,
}

impl GroupTypeFlag {
    /// Returns the raw u32 value
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Checks if a group type value contains this flag
    pub fn is_set_in(self, group_type: u32) -> bool {
        (group_type & self.as_u32()) != 0
    }

    /// Returns all flags set in a group type value
    pub fn from_value(group_type: u32) -> Vec<GroupTypeFlag> {
        let all_flags = [
            GroupTypeFlag::CreatedBySystem,
            GroupTypeFlag::GlobalScope,
            GroupTypeFlag::DomainLocalScope,
            GroupTypeFlag::UniversalScope,
            GroupTypeFlag::AppBasic,
            GroupTypeFlag::AppQuery,
            GroupTypeFlag::Security,
        ];

        all_flags
            .iter()
            .filter(|flag| flag.is_set_in(group_type))
            .copied()
            .collect()
    }

    /// Combines multiple flags into a single u32 value
    pub fn combine(flags: &[GroupTypeFlag]) -> u32 {
        flags.iter().fold(0, |acc, flag| acc | flag.as_u32())
    }

    /// Returns true if this is a security-enabled group
    pub fn is_security_enabled(group_type: u32) -> bool {
        GroupTypeFlag::Security.is_set_in(group_type)
    }

    /// Returns true if this is a distribution group (non-security)
    pub fn is_distribution(group_type: u32) -> bool {
        !Self::is_security_enabled(group_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_type_flag_values() {
        assert_eq!(GroupTypeFlag::CreatedBySystem as u32, 0x1);
        assert_eq!(GroupTypeFlag::GlobalScope as u32, 0x2);
        assert_eq!(GroupTypeFlag::DomainLocalScope as u32, 0x4);
        assert_eq!(GroupTypeFlag::UniversalScope as u32, 0x8);
        assert_eq!(GroupTypeFlag::Security as u32, 0x80000000);
    }

    #[test]
    fn test_group_type_flag_is_set_in() {
        // Global security group: 0x80000002
        let group_type = 0x80000002;
        assert!(GroupTypeFlag::GlobalScope.is_set_in(group_type));
        assert!(GroupTypeFlag::Security.is_set_in(group_type));
        assert!(!GroupTypeFlag::DomainLocalScope.is_set_in(group_type));
    }

    #[test]
    fn test_group_type_flag_from_value() {
        // Universal security group: 0x80000008
        let group_type = 0x80000008;
        let flags = GroupTypeFlag::from_value(group_type);
        assert_eq!(flags.len(), 2);
        assert!(flags.contains(&GroupTypeFlag::UniversalScope));
        assert!(flags.contains(&GroupTypeFlag::Security));
    }

    #[test]
    fn test_group_type_flag_combine() {
        let flags = vec![GroupTypeFlag::GlobalScope, GroupTypeFlag::Security];
        let combined = GroupTypeFlag::combine(&flags);
        assert_eq!(combined, 0x80000002);
    }

    #[test]
    fn test_group_type_is_security_enabled() {
        assert!(GroupTypeFlag::is_security_enabled(0x80000002)); // Global security
        assert!(!GroupTypeFlag::is_security_enabled(0x00000002)); // Global distribution
    }

    #[test]
    fn test_group_type_is_distribution() {
        assert!(GroupTypeFlag::is_distribution(0x00000002)); // Global distribution
        assert!(!GroupTypeFlag::is_distribution(0x80000002)); // Global security
    }

    #[test]
    fn test_typical_security_groups() {
        // Domain local security: 0x80000004
        let dl_security = 0x80000004;
        assert!(GroupTypeFlag::DomainLocalScope.is_set_in(dl_security));
        assert!(GroupTypeFlag::is_security_enabled(dl_security));

        // Universal security: 0x80000008
        let u_security = 0x80000008;
        assert!(GroupTypeFlag::UniversalScope.is_set_in(u_security));
        assert!(GroupTypeFlag::is_security_enabled(u_security));
    }
}
