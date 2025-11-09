//! GPT template information

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// GPT template
///
/// Represents a Group Policy Template (GptTmpl.inf) file.
/// In the C# version, this extends Dictionary<string, Dictionary<string, object>>.
/// In Rust, we use a HashMap wrapper with an additional Path field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GptTmpl {
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

impl GptTmpl {
    /// Creates a new empty GptTmpl
    pub fn new() -> Self {
        Self {
            path: None,
            data: HashMap::new(),
        }
    }

    /// Creates a GptTmpl from existing data
    pub fn from_data(data: HashMap<String, HashMap<String, Value>>) -> Self {
        Self {
            path: None,
            data,
        }
    }

    /// Gets a section from the template
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

impl Default for GptTmpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_gpt_tmpl_new() {
        let tmpl = GptTmpl::new();
        assert_eq!(tmpl.path, None);
        assert!(tmpl.data.is_empty());
    }

    #[test]
    fn test_gpt_tmpl_insert_section() {
        let mut tmpl = GptTmpl::new();
        let mut system_access = HashMap::new();
        system_access.insert("MinimumPasswordLength".to_string(), json!(8));
        system_access.insert("PasswordComplexity".to_string(), json!(1));

        tmpl.insert_section("System Access".to_string(), system_access);

        assert_eq!(tmpl.data.len(), 1);
        assert!(tmpl.get_section("System Access").is_some());
    }

    #[test]
    fn test_gpt_tmpl_get_value() {
        let mut tmpl = GptTmpl::new();
        let mut system_access = HashMap::new();
        system_access.insert("MinimumPasswordAge".to_string(), json!(1));

        tmpl.insert_section("System Access".to_string(), system_access);

        let value = tmpl.get_value("System Access", "MinimumPasswordAge");
        assert_eq!(value, Some(&json!(1)));
    }

    #[test]
    fn test_gpt_tmpl_serde() {
        let mut tmpl = GptTmpl::new();
        tmpl.path = Some("C:\\Windows\\SYSVOL\\...\\GptTmpl.inf".to_string());

        let mut privileges = HashMap::new();
        privileges.insert("SeBackupPrivilege".to_string(), json!("*S-1-5-32-544"));

        tmpl.insert_section("Privilege Rights".to_string(), privileges);

        let json = serde_json::to_string(&tmpl).unwrap();
        assert!(json.contains("Path"));
        assert!(json.contains("Privilege Rights"));

        let deserialized: GptTmpl = serde_json::from_str(&json).unwrap();
        assert_eq!(tmpl, deserialized);
    }
}
