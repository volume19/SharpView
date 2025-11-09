//! Groups XML information

use crate::returns::Filter;
use serde::{Deserialize, Serialize};

/// Groups XML
///
/// Represents group configuration from a GPO's Groups.xml file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupsXml {
    /// The GPO path
    #[serde(rename = "GPOPath", skip_serializing_if = "Option::is_none")]
    pub gpo_path: Option<String>,

    /// Filters applied to the group
    #[serde(rename = "Filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,

    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The group SID
    #[serde(rename = "GroupSID", skip_serializing_if = "Option::is_none")]
    pub group_sid: Option<String>,

    /// Groups this group is a member of
    #[serde(rename = "GroupMemberOf", skip_serializing_if = "Option::is_none")]
    pub group_member_of: Option<Vec<String>>,

    /// Members of this group
    #[serde(rename = "GroupMembers", skip_serializing_if = "Option::is_none")]
    pub group_members: Option<Vec<String>>,
}

impl GroupsXml {
    /// Creates a new GroupsXml
    pub fn new() -> Self {
        Self {
            gpo_path: None,
            filters: None,
            group_name: None,
            group_sid: None,
            group_member_of: None,
            group_members: None,
        }
    }
}

impl Default for GroupsXml {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groups_xml_new() {
        let groups = GroupsXml::new();
        assert_eq!(groups.group_name, None);
        assert_eq!(groups.gpo_path, None);
    }

    #[test]
    fn test_groups_xml_with_values() {
        let mut groups = GroupsXml::new();
        groups.gpo_path = Some("\\\\domain.com\\SYSVOL\\...".to_string());
        groups.group_name = Some("Local Administrators".to_string());
        groups.group_members = Some(vec!["DOMAIN\\Domain Admins".to_string()]);

        assert_eq!(groups.group_name, Some("Local Administrators".to_string()));
        assert_eq!(groups.group_members.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_groups_xml_serde() {
        let mut groups = GroupsXml::new();
        groups.group_name = Some("Test Group".to_string());
        groups.group_sid = Some("S-1-5-32-544".to_string());

        let json = serde_json::to_string(&groups).unwrap();
        assert!(json.contains("GroupName"));
        assert!(json.contains("GroupSID"));

        let deserialized: GroupsXml = serde_json::from_str(&json).unwrap();
        assert_eq!(groups, deserialized);
    }
}
