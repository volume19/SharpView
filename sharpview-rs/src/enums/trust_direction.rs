//! Trust direction enumeration from System.DirectoryServices.ActiveDirectory

use serde::{Deserialize, Serialize};

/// Domain trust direction
///
/// Specifies the direction of a trust relationship between domains.
/// Maps to System.DirectoryServices.ActiveDirectory.TrustDirection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum TrustDirection {
    /// Outbound trust - this domain trusts the other domain (1)
    Outbound = 1,

    /// Inbound trust - the other domain trusts this domain (2)
    Inbound = 2,

    /// Bidirectional trust - both domains trust each other (3)
    Bidirectional = 3,
}

impl TrustDirection {
    /// Creates a TrustDirection from a raw u32 value
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            1 => Some(Self::Outbound),
            2 => Some(Self::Inbound),
            3 => Some(Self::Bidirectional),
            _ => None,
        }
    }

    /// Checks if the trust includes outbound direction
    pub fn is_outbound(&self) -> bool {
        matches!(self, Self::Outbound | Self::Bidirectional)
    }

    /// Checks if the trust includes inbound direction
    pub fn is_inbound(&self) -> bool {
        matches!(self, Self::Inbound | Self::Bidirectional)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_direction_values() {
        assert_eq!(TrustDirection::Outbound as u32, 1);
        assert_eq!(TrustDirection::Inbound as u32, 2);
        assert_eq!(TrustDirection::Bidirectional as u32, 3);
    }

    #[test]
    fn test_trust_direction_from_value() {
        assert_eq!(TrustDirection::from_value(1), Some(TrustDirection::Outbound));
        assert_eq!(TrustDirection::from_value(3), Some(TrustDirection::Bidirectional));
        assert_eq!(TrustDirection::from_value(99), None);
    }

    #[test]
    fn test_trust_direction_helpers() {
        assert!(TrustDirection::Outbound.is_outbound());
        assert!(!TrustDirection::Outbound.is_inbound());

        assert!(!TrustDirection::Inbound.is_outbound());
        assert!(TrustDirection::Inbound.is_inbound());

        assert!(TrustDirection::Bidirectional.is_outbound());
        assert!(TrustDirection::Bidirectional.is_inbound());
    }

    #[test]
    fn test_trust_direction_serde() {
        let direction = TrustDirection::Bidirectional;
        let json = serde_json::to_string(&direction).unwrap();
        assert_eq!(json, "\"Bidirectional\"");

        let deserialized: TrustDirection = serde_json::from_str(&json).unwrap();
        assert_eq!(direction, deserialized);
    }
}
