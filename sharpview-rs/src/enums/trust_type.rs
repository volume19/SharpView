//! Trust type enumeration from System.DirectoryServices.ActiveDirectory

use serde::{Deserialize, Serialize};

/// Domain trust type
///
/// Specifies the type of trust relationship between domains.
/// Maps to System.DirectoryServices.ActiveDirectory.TrustType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum TrustType {
    /// Tree root trust (0)
    TreeRoot = 0,

    /// Parent-child trust (1)
    ParentChild = 1,

    /// Cross-link trust (2)
    CrossLink = 2,

    /// External trust (3)
    External = 3,

    /// Forest trust (4)
    Forest = 4,

    /// Kerberos trust (5)
    Kerberos = 5,

    /// Unknown trust type (6)
    Unknown = 6,
}

impl TrustType {
    /// Creates a TrustType from a raw u32 value
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::TreeRoot),
            1 => Some(Self::ParentChild),
            2 => Some(Self::CrossLink),
            3 => Some(Self::External),
            4 => Some(Self::Forest),
            5 => Some(Self::Kerberos),
            6 => Some(Self::Unknown),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_type_values() {
        assert_eq!(TrustType::TreeRoot as u32, 0);
        assert_eq!(TrustType::ParentChild as u32, 1);
        assert_eq!(TrustType::Forest as u32, 4);
        assert_eq!(TrustType::Unknown as u32, 6);
    }

    #[test]
    fn test_trust_type_from_value() {
        assert_eq!(TrustType::from_value(0), Some(TrustType::TreeRoot));
        assert_eq!(TrustType::from_value(4), Some(TrustType::Forest));
        assert_eq!(TrustType::from_value(99), None);
    }

    #[test]
    fn test_trust_type_serde() {
        let trust_type = TrustType::Forest;
        let json = serde_json::to_string(&trust_type).unwrap();
        assert_eq!(json, "\"Forest\"");

        let deserialized: TrustType = serde_json::from_str(&json).unwrap();
        assert_eq!(trust_type, deserialized);
    }
}
