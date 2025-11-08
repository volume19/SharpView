//! DS_DOMAIN_TRUST_TYPE enumeration from Windows API

use serde::{Deserialize, Serialize};

/// DS domain trust type flags
///
/// Bitflags for domain trust types from the Windows DsEnumerateDomainTrusts API.
/// Maps to DS_DOMAIN_TRUST_TYPE from the Windows SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum DsTrustType {
    /// Domain is a member of the forest
    InForest = 0x0001,

    /// Domain is directly trusted (outbound)
    DirectOutbound = 0x0002,

    /// Domain is root of a tree in the forest
    TreeRoot = 0x0004,

    /// Domain is the primary domain of queried server
    Primary = 0x0008,

    /// Primary domain is running in native mode
    NativeMode = 0x0010,

    /// Domain is directly trusting (inbound)
    DirectInbound = 0x0020,
}

impl DsTrustType {
    /// Checks if this flag is set in the given trust type value
    pub fn is_set_in(self, trust_type: u32) -> bool {
        (trust_type & (self as u32)) != 0
    }

    /// Extracts all set flags from a trust type value
    pub fn from_value(trust_type: u32) -> Vec<DsTrustType> {
        let all_flags = [
            DsTrustType::InForest,
            DsTrustType::DirectOutbound,
            DsTrustType::TreeRoot,
            DsTrustType::Primary,
            DsTrustType::NativeMode,
            DsTrustType::DirectInbound,
        ];

        all_flags
            .iter()
            .filter(|flag| flag.is_set_in(trust_type))
            .copied()
            .collect()
    }

    /// Combines multiple flags into a single value
    pub fn combine(flags: &[DsTrustType]) -> u32 {
        flags.iter().fold(0u32, |acc, flag| acc | (*flag as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ds_trust_type_values() {
        assert_eq!(DsTrustType::InForest as u32, 0x0001);
        assert_eq!(DsTrustType::DirectOutbound as u32, 0x0002);
        assert_eq!(DsTrustType::TreeRoot as u32, 0x0004);
        assert_eq!(DsTrustType::Primary as u32, 0x0008);
        assert_eq!(DsTrustType::NativeMode as u32, 0x0010);
        assert_eq!(DsTrustType::DirectInbound as u32, 0x0020);
    }

    #[test]
    fn test_ds_trust_type_is_set_in() {
        let value = 0x0005; // InForest | TreeRoot
        assert!(DsTrustType::InForest.is_set_in(value));
        assert!(!DsTrustType::DirectOutbound.is_set_in(value));
        assert!(DsTrustType::TreeRoot.is_set_in(value));
        assert!(!DsTrustType::Primary.is_set_in(value));
    }

    #[test]
    fn test_ds_trust_type_from_value() {
        let value = 0x0005; // InForest | TreeRoot
        let flags = DsTrustType::from_value(value);
        assert_eq!(flags.len(), 2);
        assert!(flags.contains(&DsTrustType::InForest));
        assert!(flags.contains(&DsTrustType::TreeRoot));
    }

    #[test]
    fn test_ds_trust_type_combine() {
        let flags = vec![DsTrustType::InForest, DsTrustType::TreeRoot];
        let combined = DsTrustType::combine(&flags);
        assert_eq!(combined, 0x0005);
    }

    #[test]
    fn test_ds_trust_type_serde() {
        let flag = DsTrustType::InForest;
        let json = serde_json::to_string(&flag).unwrap();
        assert_eq!(json, "\"InForest\"");

        let deserialized: DsTrustType = serde_json::from_str(&json).unwrap();
        assert_eq!(flag, deserialized);
    }
}
