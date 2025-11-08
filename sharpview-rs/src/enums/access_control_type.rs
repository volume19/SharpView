//! Access control type enumeration

use serde::{Deserialize, Serialize};

/// Access control type
///
/// Specifies whether an access control entry (ACE) allows or denies access.
/// Maps to System.Security.AccessControl.AccessControlType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum AccessControlType {
    /// Access is allowed (0)
    Allow = 0,

    /// Access is denied (1)
    Deny = 1,
}

impl AccessControlType {
    /// Creates an AccessControlType from a raw u32 value
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Allow),
            1 => Some(Self::Deny),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control_type_values() {
        assert_eq!(AccessControlType::Allow as u32, 0);
        assert_eq!(AccessControlType::Deny as u32, 1);
    }

    #[test]
    fn test_access_control_type_from_value() {
        assert_eq!(AccessControlType::from_value(0), Some(AccessControlType::Allow));
        assert_eq!(AccessControlType::from_value(1), Some(AccessControlType::Deny));
        assert_eq!(AccessControlType::from_value(99), None);
    }

    #[test]
    fn test_access_control_type_serde() {
        let acl_type = AccessControlType::Allow;
        let json = serde_json::to_string(&acl_type).unwrap();
        assert_eq!(json, "\"Allow\"");

        let deserialized: AccessControlType = serde_json::from_str(&json).unwrap();
        assert_eq!(acl_type, deserialized);
    }
}
