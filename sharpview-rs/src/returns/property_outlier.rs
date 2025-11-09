//! Property outlier information

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Property outlier
///
/// Represents an outlier property value for an object, used to identify
/// anomalous configurations in Active Directory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyOutlier {
    /// The SAM account name
    #[serde(rename = "SamAccountName", skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,

    /// The property name
    #[serde(rename = "Property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    /// The property value (can be any type)
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl PropertyOutlier {
    /// Creates a new PropertyOutlier
    pub fn new(
        sam_account_name: Option<String>,
        property: Option<String>,
        value: Option<Value>,
    ) -> Self {
        Self {
            sam_account_name,
            property,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_property_outlier_new() {
        let outlier = PropertyOutlier::new(
            Some("admin".to_string()),
            Some("logonCount".to_string()),
            Some(json!(0)),
        );
        assert_eq!(outlier.sam_account_name, Some("admin".to_string()));
        assert_eq!(outlier.property, Some("logonCount".to_string()));
    }

    #[test]
    fn test_property_outlier_with_string_value() {
        let outlier = PropertyOutlier::new(
            Some("user1".to_string()),
            Some("homeDirectory".to_string()),
            Some(json!("\\\\server\\share")),
        );
        assert_eq!(outlier.value, Some(json!("\\\\server\\share")));
    }

    #[test]
    fn test_property_outlier_serde() {
        let outlier = PropertyOutlier::new(
            Some("testuser".to_string()),
            Some("badPwdCount".to_string()),
            Some(json!(10)),
        );

        let json = serde_json::to_string(&outlier).unwrap();
        assert!(json.contains("SamAccountName"));
        assert!(json.contains("Property"));
        assert!(json.contains("Value"));

        let deserialized: PropertyOutlier = serde_json::from_str(&json).unwrap();
        assert_eq!(outlier, deserialized);
    }
}
