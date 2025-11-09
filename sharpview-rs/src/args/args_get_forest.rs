//! Arguments for Get-Forest operations

use crate::args::NetworkCredential;
use serde::{Deserialize, Serialize};

/// Arguments for Get-Forest
///
/// Arguments for forest enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetForest {
    /// The target forest
    #[serde(rename = "Forest", skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetForest {
    pub fn new() -> Self {
        Self {
            forest: None,
            credential: None,
        }
    }

    pub fn forest(mut self, forest: String) -> Self {
        self.forest = Some(forest);
        self
    }

    pub fn name(self, name: String) -> Self {
        self.forest(name)
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetForest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_forest_new() {
        let args = ArgsGetForest::new();
        assert_eq!(args.forest, None);
    }

    #[test]
    fn test_args_get_forest_builder() {
        let args = ArgsGetForest::new()
            .forest("corp.local".to_string());

        assert_eq!(args.forest, Some("corp.local".to_string()));
    }

    #[test]
    fn test_args_get_forest_serde() {
        let args = ArgsGetForest::new()
            .forest("test.forest".to_string());

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("Forest"));

        let deserialized: ArgsGetForest = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
