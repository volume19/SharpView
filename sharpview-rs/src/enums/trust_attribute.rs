//! Domain trust attribute flags
//!
//! Maps to trustAttributes attribute values for domain trust relationships.

use serde::{Deserialize, Serialize};

/// Trust attribute flags
///
/// Bitflags representing characteristics of Active Directory domain trusts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum TrustAttributeFlag {
    /// Trust is non-transitive
    NonTransitive = 0x00000001,

    /// Trust is with a Windows 2000 or later domain
    UplevelOnly = 0x00000002,

    /// SID filtering is enabled (quarantined domain)
    FilterSids = 0x00000004,

    /// Trust is a forest trust
    ForestTransitive = 0x00000008,

    /// Trust is a cross-organization trust (selective authentication)
    CrossOrganization = 0x00000010,

    /// Trust is within the same forest
    WithinForest = 0x00000020,

    /// Trust should be treated as an external trust for SID filtering
    TreatAsExternal = 0x00000040,

    /// Trust uses RC4 encryption for keys
    TrustUsesRC4Encryption = 0x00000080,

    /// Trust uses AES encryption for keys
    TrustUsesAESKeys = 0x00000100,

    /// Cross-organization trust without TGT delegation
    CrossOrganizationNoTgtDelegation = 0x00000200,

    /// Privileged Identity Management (PIM) trust
    PimTrust = 0x00000400,
}

impl TrustAttributeFlag {
    /// Returns the raw u32 value
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Checks if a trust attribute value contains this flag
    pub fn is_set_in(self, trust_attributes: u32) -> bool {
        (trust_attributes & self.as_u32()) != 0
    }

    /// Returns all flags set in a trust attribute value
    pub fn from_value(trust_attributes: u32) -> Vec<TrustAttributeFlag> {
        let all_flags = [
            TrustAttributeFlag::NonTransitive,
            TrustAttributeFlag::UplevelOnly,
            TrustAttributeFlag::FilterSids,
            TrustAttributeFlag::ForestTransitive,
            TrustAttributeFlag::CrossOrganization,
            TrustAttributeFlag::WithinForest,
            TrustAttributeFlag::TreatAsExternal,
            TrustAttributeFlag::TrustUsesRC4Encryption,
            TrustAttributeFlag::TrustUsesAESKeys,
            TrustAttributeFlag::CrossOrganizationNoTgtDelegation,
            TrustAttributeFlag::PimTrust,
        ];

        all_flags
            .iter()
            .filter(|flag| flag.is_set_in(trust_attributes))
            .copied()
            .collect()
    }

    /// Combines multiple flags into a single u32 value
    pub fn combine(flags: &[TrustAttributeFlag]) -> u32 {
        flags.iter().fold(0, |acc, flag| acc | flag.as_u32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_attribute_flag_values() {
        assert_eq!(TrustAttributeFlag::NonTransitive as u32, 0x1);
        assert_eq!(TrustAttributeFlag::ForestTransitive as u32, 0x8);
        assert_eq!(TrustAttributeFlag::WithinForest as u32, 0x20);
        assert_eq!(TrustAttributeFlag::TrustUsesAESKeys as u32, 0x100);
        assert_eq!(TrustAttributeFlag::PimTrust as u32, 0x400);
    }

    #[test]
    fn test_trust_attribute_flag_is_set_in() {
        // Forest trust with AES: 0x108
        let trust_attrs = 0x108;
        assert!(TrustAttributeFlag::ForestTransitive.is_set_in(trust_attrs));
        assert!(TrustAttributeFlag::TrustUsesAESKeys.is_set_in(trust_attrs));
        assert!(!TrustAttributeFlag::FilterSids.is_set_in(trust_attrs));
    }

    #[test]
    fn test_trust_attribute_flag_from_value() {
        let trust_attrs = 0x28; // ForestTransitive | WithinForest
        let flags = TrustAttributeFlag::from_value(trust_attrs);
        assert_eq!(flags.len(), 2);
        assert!(flags.contains(&TrustAttributeFlag::ForestTransitive));
        assert!(flags.contains(&TrustAttributeFlag::WithinForest));
    }

    #[test]
    fn test_trust_attribute_flag_combine() {
        let flags = vec![
            TrustAttributeFlag::ForestTransitive,
            TrustAttributeFlag::TrustUsesAESKeys,
        ];
        let combined = TrustAttributeFlag::combine(&flags);
        assert_eq!(combined, 0x108);
    }

    #[test]
    fn test_within_forest_trust() {
        // Typical within-forest trust: WithinForest | ForestTransitive
        let trust_attrs = 0x28;
        assert!(TrustAttributeFlag::WithinForest.is_set_in(trust_attrs));
        assert!(TrustAttributeFlag::ForestTransitive.is_set_in(trust_attrs));
    }

    #[test]
    fn test_external_trust_with_sid_filtering() {
        // External trust with SID filtering: FilterSids | TreatAsExternal
        let trust_attrs = 0x44;
        assert!(TrustAttributeFlag::FilterSids.is_set_in(trust_attrs));
        assert!(TrustAttributeFlag::TreatAsExternal.is_set_in(trust_attrs));
    }
}
