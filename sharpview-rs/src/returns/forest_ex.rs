//! Forest extended information

use serde::{Deserialize, Serialize};

/// Forest extended
///
/// Represents extended information about an Active Directory forest.
/// Note: In the C# version, this contains a System.DirectoryServices.ActiveDirectory.Forest
/// object. In Rust, we represent the forest information as strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForestEx {
    /// The forest name or information as a string
    #[serde(rename = "Forest", skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,

    /// The root domain SID
    #[serde(rename = "RootDomainSid", skip_serializing_if = "Option::is_none")]
    pub root_domain_sid: Option<String>,
}

impl ForestEx {
    /// Creates a new ForestEx
    pub fn new(forest: Option<String>, root_domain_sid: Option<String>) -> Self {
        Self {
            forest,
            root_domain_sid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forest_ex_new() {
        let forest = ForestEx::new(
            Some("CORP.COM".to_string()),
            Some("S-1-5-21-1234567890-1234567890-1234567890".to_string()),
        );
        assert_eq!(forest.forest, Some("CORP.COM".to_string()));
        assert_eq!(forest.root_domain_sid, Some("S-1-5-21-1234567890-1234567890-1234567890".to_string()));
    }

    #[test]
    fn test_forest_ex_serde() {
        let forest = ForestEx::new(
            Some("DOMAIN.LOCAL".to_string()),
            Some("S-1-5-21-123-456-789".to_string()),
        );

        let json = serde_json::to_string(&forest).unwrap();
        assert!(json.contains("Forest"));
        assert!(json.contains("RootDomainSid"));

        let deserialized: ForestEx = serde_json::from_str(&json).unwrap();
        assert_eq!(forest, deserialized);
    }
}
