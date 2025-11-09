//! Filter information

use serde::{Deserialize, Serialize};

/// Filter
///
/// Represents a filter with a type and value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    /// The filter type
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,

    /// The filter value
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Filter {
    /// Creates a new Filter
    pub fn new(filter_type: Option<String>, value: Option<String>) -> Self {
        Self {
            filter_type,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_new() {
        let filter = Filter::new(
            Some("LDAP".to_string()),
            Some("(objectClass=user)".to_string()),
        );
        assert_eq!(filter.filter_type, Some("LDAP".to_string()));
        assert_eq!(filter.value, Some("(objectClass=user)".to_string()));
    }

    #[test]
    fn test_filter_serde() {
        let filter = Filter::new(
            Some("WQL".to_string()),
            Some("SELECT * FROM Win32_Process".to_string()),
        );

        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("Type"));
        assert!(json.contains("Value"));

        let deserialized: Filter = serde_json::from_str(&json).unwrap();
        assert_eq!(filter, deserialized);
    }
}
