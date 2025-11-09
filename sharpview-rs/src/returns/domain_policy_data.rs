//! Domain policy data information

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Domain policy data
///
/// Represents domain policy data from a GPT template.
/// In the C# version, this extends GptTmpl which extends Dictionary.
/// In Rust, we combine GptTmpl's fields with additional GPO fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainPolicyData {
    /// The GPO name
    #[serde(rename = "GPOName", skip_serializing_if = "Option::is_none")]
    pub gpo_name: Option<String>,

    /// The GPO display name
    #[serde(rename = "GPODisplayName", skip_serializing_if = "Option::is_none")]
    pub gpo_display_name: Option<String>,

    /// The path to the GPT template file
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// The template data as a nested hashmap
    /// Outer key: section name (e.g., "System Access", "Privilege Rights")
    /// Inner key: setting name
    /// Inner value: setting value (can be any type)
    #[serde(flatten)]
    pub data: HashMap<String, HashMap<String, Value>>,
}

impl DomainPolicyData {
    /// Creates a new empty DomainPolicyData
    pub fn new() -> Self {
        Self {
            gpo_name: None,
            gpo_display_name: None,
            path: None,
            data: HashMap::new(),
        }
    }

    /// Creates a DomainPolicyData from existing data
    pub fn from_data(data: HashMap<String, HashMap<String, Value>>) -> Self {
        Self {
            gpo_name: None,
            gpo_display_name: None,
            path: None,
            data,
        }
    }

    /// Gets a section from the policy
    pub fn get_section(&self, section: &str) -> Option<&HashMap<String, Value>> {
        self.data.get(section)
    }

    /// Gets a value from a specific section and setting
    pub fn get_value(&self, section: &str, setting: &str) -> Option<&Value> {
        self.data.get(section)?.get(setting)
    }

    /// Inserts a new section
    pub fn insert_section(&mut self, section: String, values: HashMap<String, Value>) {
        self.data.insert(section, values);
    }
}

impl Default for DomainPolicyData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_domain_policy_data_new() {
        let policy = DomainPolicyData::new();
        assert_eq!(policy.gpo_name, None);
        assert_eq!(policy.gpo_display_name, None);
        assert!(policy.data.is_empty());
    }

    #[test]
    fn test_domain_policy_data_with_gpo_info() {
        let mut policy = DomainPolicyData::new();
        policy.gpo_name = Some("{31B2F340-016D-11D2-945F-00C04FB984F9}".to_string());
        policy.gpo_display_name = Some("Default Domain Policy".to_string());

        assert_eq!(policy.gpo_display_name, Some("Default Domain Policy".to_string()));
    }

    #[test]
    fn test_domain_policy_data_insert_section() {
        let mut policy = DomainPolicyData::new();
        let mut system_access = HashMap::new();
        system_access.insert("MinimumPasswordLength".to_string(), json!(8));
        system_access.insert("PasswordComplexity".to_string(), json!(1));
        system_access.insert("MaximumPasswordAge".to_string(), json!(42));

        policy.insert_section("System Access".to_string(), system_access);

        assert_eq!(policy.data.len(), 1);
        assert!(policy.get_section("System Access").is_some());
    }

    #[test]
    fn test_domain_policy_data_get_value() {
        let mut policy = DomainPolicyData::new();
        let mut kerberos_policy = HashMap::new();
        kerberos_policy.insert("MaxTicketAge".to_string(), json!(10));

        policy.insert_section("Kerberos Policy".to_string(), kerberos_policy);

        let value = policy.get_value("Kerberos Policy", "MaxTicketAge");
        assert_eq!(value, Some(&json!(10)));
    }

    #[test]
    fn test_domain_policy_data_serde() {
        let mut policy = DomainPolicyData::new();
        policy.gpo_name = Some("{6AC1786C-016F-11D2-945F-00C04fB984F9}".to_string());
        policy.gpo_display_name = Some("Default Domain Controllers Policy".to_string());

        let mut audit_policy = HashMap::new();
        audit_policy.insert("AuditSystemEvents".to_string(), json!(1));

        policy.insert_section("Event Audit".to_string(), audit_policy);

        let json = serde_json::to_string(&policy).unwrap();
        assert!(json.contains("GPOName"));
        assert!(json.contains("GPODisplayName"));
        assert!(json.contains("Event Audit"));

        let deserialized: DomainPolicyData = serde_json::from_str(&json).unwrap();
        assert_eq!(policy, deserialized);
    }
}
