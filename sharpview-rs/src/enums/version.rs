//! Version enumeration for protocol or schema versions

use serde::{Deserialize, Serialize};

/// Version selector
///
/// Specifies which version(s) to target for operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Version {
    /// All versions
    All,

    /// Version 1
    V1,

    /// Version 2
    V2,
}

impl Default for Version {
    fn default() -> Self {
        Self::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_default() {
        assert_eq!(Version::default(), Version::All);
    }

    #[test]
    fn test_version_variants() {
        let versions = vec![Version::All, Version::V1, Version::V2];
        assert_eq!(versions.len(), 3);
    }

    #[test]
    fn test_version_serde() {
        let version = Version::V1;
        let json = serde_json::to_string(&version).unwrap();
        assert_eq!(json, "\"V1\"");

        let deserialized: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(version, deserialized);
    }
}
