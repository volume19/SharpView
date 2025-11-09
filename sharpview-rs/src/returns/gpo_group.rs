//! GPO group information

use crate::returns::Filter;
use serde::{Deserialize, Serialize};

/// GPO group
///
/// Represents group information from a Group Policy Object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpoGroup {
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
}

impl GpoGroup {
    /// Creates a new GpoGroup
    pub fn new() -> Self {
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
        }
    }
}

impl Default for GpoGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpo_group_new() {
        let gpo_group = GpoGroup::new();
        assert_eq!(gpo_group.gpo_display_name, None);
        assert_eq!(gpo_group.group_name, None);
    }

    #[test]
    fn test_gpo_group_with_values() {
        let mut gpo_group = GpoGroup::new();
        gpo_group.gpo_display_name = Some("Default Domain Policy".to_string());
        gpo_group.group_name = Some("Administrators".to_string());
        gpo_group.group_members = Some(vec!["Domain Admins".to_string()]);

        assert_eq!(gpo_group.gpo_display_name, Some("Default Domain Policy".to_string()));
        assert_eq!(gpo_group.group_members.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_gpo_group_serde() {
        let mut gpo_group = GpoGroup::new();
        gpo_group.gpo_name = Some("{31B2F340-016D-11D2-945F-00C04FB984F9}".to_string());
        gpo_group.group_name = Some("Remote Desktop Users".to_string());

        let json = serde_json::to_string(&gpo_group).unwrap();
        assert!(json.contains("GPOName"));
        assert!(json.contains("GroupName"));

        let deserialized: GpoGroup = serde_json::from_str(&json).unwrap();
        assert_eq!(gpo_group, deserialized);
    }
}
