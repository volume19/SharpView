//! Security masks for LDAP directory operations

use serde::{Deserialize, Serialize};

/// Security masks for directory searches
///
/// Specifies which parts of a security descriptor to retrieve.
/// Maps to System.DirectoryServices.SecurityMasks in .NET.
/// This is a flags enum that can be combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum SecurityMasks {
    /// No security descriptor information
    #[serde(rename = "None")]
    None = 0,

    /// Owner information
    #[serde(rename = "Owner")]
    Owner = 1,

    /// Group information
    #[serde(rename = "Group")]
    Group = 2,

    /// Discretionary Access Control List (DACL)
    #[serde(rename = "Dacl")]
    Dacl = 4,

    /// System Access Control List (SACL)
    #[serde(rename = "Sacl")]
    Sacl = 8,
}

impl SecurityMasks {
    /// Combines multiple security masks using bitwise OR
    pub fn combine(masks: &[SecurityMasks]) -> u32 {
        masks.iter().fold(0u32, |acc, &mask| acc | mask as u32)
    }

    /// Checks if this mask is set in the given value
    pub fn is_set_in(self, value: u32) -> bool {
        (value & self as u32) == self as u32
    }

    /// Creates a SecurityMasks from a u32 value (returns Dacl for most common case)
    pub fn from_value(value: u32) -> Self {
        match value {
            0 => SecurityMasks::None,
            1 => SecurityMasks::Owner,
            2 => SecurityMasks::Group,
            4 => SecurityMasks::Dacl,
            8 => SecurityMasks::Sacl,
            _ => SecurityMasks::Dacl, // Default to DACL for combined/unknown values
        }
    }
}

impl Default for SecurityMasks {
    fn default() -> Self {
        SecurityMasks::Dacl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_masks_values() {
        assert_eq!(SecurityMasks::None as u32, 0);
        assert_eq!(SecurityMasks::Owner as u32, 1);
        assert_eq!(SecurityMasks::Group as u32, 2);
        assert_eq!(SecurityMasks::Dacl as u32, 4);
        assert_eq!(SecurityMasks::Sacl as u32, 8);
    }

    #[test]
    fn test_security_masks_combine() {
        let combined = SecurityMasks::combine(&[SecurityMasks::Owner, SecurityMasks::Group]);
        assert_eq!(combined, 3);

        let combined_dacl_sacl = SecurityMasks::combine(&[SecurityMasks::Dacl, SecurityMasks::Sacl]);
        assert_eq!(combined_dacl_sacl, 12);
    }

    #[test]
    fn test_security_masks_is_set_in() {
        let value = 12; // Dacl (4) | Sacl (8)
        assert!(SecurityMasks::Dacl.is_set_in(value));
        assert!(SecurityMasks::Sacl.is_set_in(value));
        assert!(!SecurityMasks::Owner.is_set_in(value));
        assert!(!SecurityMasks::Group.is_set_in(value));
    }

    #[test]
    fn test_security_masks_from_value() {
        assert_eq!(SecurityMasks::from_value(0), SecurityMasks::None);
        assert_eq!(SecurityMasks::from_value(1), SecurityMasks::Owner);
        assert_eq!(SecurityMasks::from_value(4), SecurityMasks::Dacl);
        assert_eq!(SecurityMasks::from_value(8), SecurityMasks::Sacl);
        assert_eq!(SecurityMasks::from_value(99), SecurityMasks::Dacl); // Unknown defaults to Dacl
    }

    #[test]
    fn test_security_masks_default() {
        assert_eq!(SecurityMasks::default(), SecurityMasks::Dacl);
    }

    #[test]
    fn test_security_masks_serde() {
        let mask = SecurityMasks::Sacl;
        let json = serde_json::to_string(&mask).unwrap();
        assert_eq!(json, "\"Sacl\"");

        let deserialized: SecurityMasks = serde_json::from_str(&json).unwrap();
        assert_eq!(mask, deserialized);
    }
}
