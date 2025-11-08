//! Stealth enumeration source type

use serde::{Deserialize, Serialize};

/// Stealth source type
///
/// Specifies the source for stealth enumeration operations to minimize detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StealthSource {
    /// Use DFS shares for enumeration
    #[serde(rename = "DFS")]
    Dfs,

    /// Use domain controllers for enumeration
    #[serde(rename = "DC")]
    Dc,

    /// Use file servers for enumeration
    File,

    /// Use all available sources
    All,
}

impl Default for StealthSource {
    fn default() -> Self {
        Self::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stealth_source_default() {
        assert_eq!(StealthSource::default(), StealthSource::All);
    }

    #[test]
    fn test_stealth_source_serde() {
        let source = StealthSource::Dfs;
        let json = serde_json::to_string(&source).unwrap();
        assert_eq!(json, "\"DFS\"");

        let deserialized: StealthSource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, deserialized);
    }

    #[test]
    fn test_stealth_source_all_variants() {
        let test_cases = vec![
            (StealthSource::Dfs, "\"DFS\""),
            (StealthSource::Dc, "\"DC\""),
            (StealthSource::File, "\"File\""),
            (StealthSource::All, "\"All\""),
        ];

        for (source, expected_json) in test_cases {
            let json = serde_json::to_string(&source).unwrap();
            assert_eq!(json, expected_json);

            let deserialized: StealthSource = serde_json::from_str(&json).unwrap();
            assert_eq!(source, deserialized);
        }
    }
}
