//! GPO local group mapping information

use crate::returns::Filter;
use serde::{Deserialize, Serialize};

/// GPO local group mapping
///
/// Represents the mapping between a GPO-configured local group and its members.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpoLocalGroupMapping {
    /// The GPO display name
    #[serde(rename = "GPODisplayName", skip_serializing_if = "Option::is_none")]
    pub gpo_display_name: Option<String>,

    /// The GPO name
    #[serde(rename = "GPOName", skip_serializing_if = "Option::is_none")]
    pub gpo_name: Option<String>,

    /// The GPO path
    #[serde(rename = "GPOPath", skip_serializing_if = "Option::is_none")]
    pub gpo_path: Option<String>,

    /// The GPO type
    #[serde(rename = "GPOType", skip_serializing_if = "Option::is_none")]
    pub gpo_type: Option<String>,

    /// Filters applied to the GPO
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

    /// The object name
    #[serde(rename = "ObjectName", skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,

    /// The object distinguished name
    #[serde(rename = "ObjectDN", skip_serializing_if = "Option::is_none")]
    pub object_dn: Option<String>,

    /// The object SID(s)
    #[serde(rename = "ObjectSID", skip_serializing_if = "Option::is_none")]
    pub object_sid: Option<Vec<String>>,

    /// The domain
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Whether the object is a group
    #[serde(rename = "IsGroup")]
    pub is_group: bool,

    /// The GPO GUID
    #[serde(rename = "GPOGuid", skip_serializing_if = "Option::is_none")]
    pub gpo_guid: Option<String>,

    /// The container name
    #[serde(rename = "ContainerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// The computer names
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<Vec<String>>,
}

impl GpoLocalGroupMapping {
    /// Creates a new GpoLocalGroupMapping
    pub fn new(is_group: bool) -> Self {
        Self {
            gpo_display_name: None,
            gpo_name: None,
            gpo_path: None,
            gpo_type: None,
            filters: None,
            group_name: None,
            group_sid: None,
            group_member_of: None,
            group_members: None,
            object_name: None,
            object_dn: None,
            object_sid: None,
            domain: None,
            is_group,
            gpo_guid: None,
            container_name: None,
            computer_name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpo_local_group_mapping_new() {
        let mapping = GpoLocalGroupMapping::new(false);
        assert!(!mapping.is_group);
        assert_eq!(mapping.gpo_display_name, None);
    }

    #[test]
    fn test_gpo_local_group_mapping_with_values() {
        let mut mapping = GpoLocalGroupMapping::new(true);
        mapping.gpo_display_name = Some("Default Domain Policy".to_string());
        mapping.group_name = Some("Administrators".to_string());
        mapping.object_name = Some("Domain Admins".to_string());
        mapping.computer_name = Some(vec!["DC01".to_string(), "DC02".to_string()]);

        assert!(mapping.is_group);
        assert_eq!(mapping.computer_name.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_gpo_local_group_mapping_serde() {
        let mut mapping = GpoLocalGroupMapping::new(false);
        mapping.gpo_name = Some("{31B2F340-016D-11D2-945F-00C04FB984F9}".to_string());
        mapping.group_name = Some("Remote Desktop Users".to_string());
        mapping.domain = Some("CORP".to_string());

        let json = serde_json::to_string(&mapping).unwrap();
        assert!(json.contains("GPOName"));
        assert!(json.contains("GroupName"));
        assert!(json.contains("Domain"));

        let deserialized: GpoLocalGroupMapping = serde_json::from_str(&json).unwrap();
        assert_eq!(mapping, deserialized);
    }
}
