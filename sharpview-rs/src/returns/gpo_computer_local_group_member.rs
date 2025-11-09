//! GPO computer local group member information

use serde::{Deserialize, Serialize};

/// GPO computer local group member
///
/// Represents a local group member configured via Group Policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpoComputerLocalGroupMember {
    /// The computer names
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<Vec<String>>,

    /// The object name
    #[serde(rename = "ObjectName", skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,

    /// The object distinguished name
    #[serde(rename = "ObjectDN", skip_serializing_if = "Option::is_none")]
    pub object_dn: Option<String>,

    /// The object SID(s)
    #[serde(rename = "ObjectSID", skip_serializing_if = "Option::is_none")]
    pub object_sid: Option<Vec<String>>,

    /// Whether the object is a group
    #[serde(rename = "IsGroup")]
    pub is_group: bool,

    /// The GPO display name
    #[serde(rename = "GPODisplayName", skip_serializing_if = "Option::is_none")]
    pub gpo_display_name: Option<String>,

    /// The GPO GUID
    #[serde(rename = "GPOGuid", skip_serializing_if = "Option::is_none")]
    pub gpo_guid: Option<String>,

    /// The GPO path
    #[serde(rename = "GPOPath", skip_serializing_if = "Option::is_none")]
    pub gpo_path: Option<String>,

    /// The GPO type
    #[serde(rename = "GPOType", skip_serializing_if = "Option::is_none")]
    pub gpo_type: Option<String>,
}

impl GpoComputerLocalGroupMember {
    /// Creates a new GpoComputerLocalGroupMember
    pub fn new(is_group: bool) -> Self {
        Self {
            computer_name: None,
            object_name: None,
            object_dn: None,
            object_sid: None,
            is_group,
            gpo_display_name: None,
            gpo_guid: None,
            gpo_path: None,
            gpo_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpo_computer_local_group_member_new() {
        let member = GpoComputerLocalGroupMember::new(false);
        assert!(!member.is_group);
        assert_eq!(member.object_name, None);
    }

    #[test]
    fn test_gpo_computer_local_group_member_with_values() {
        let mut member = GpoComputerLocalGroupMember::new(true);
        member.computer_name = Some(vec!["WS01".to_string(), "WS02".to_string()]);
        member.object_name = Some("Domain Admins".to_string());
        member.gpo_display_name = Some("Server GPO".to_string());

        assert!(member.is_group);
        assert_eq!(member.computer_name.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_gpo_computer_local_group_member_serde() {
        let mut member = GpoComputerLocalGroupMember::new(false);
        member.object_name = Some("Administrator".to_string());
        member.object_sid = Some(vec!["S-1-5-21-...-500".to_string()]);

        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("ObjectName"));
        assert!(json.contains("IsGroup"));

        let deserialized: GpoComputerLocalGroupMember = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);
    }
}
