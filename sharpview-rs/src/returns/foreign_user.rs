//! Foreign user information

use serde::{Deserialize, Serialize};

/// Foreign user
///
/// Represents a user from a foreign (external) domain who is a member
/// of a group in the current domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForeignUser {
    /// The user's domain (foreign domain)
    #[serde(rename = "UserDomain", skip_serializing_if = "Option::is_none")]
    pub user_domain: Option<String>,

    /// The username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// The user's distinguished name
    #[serde(rename = "UserDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub user_distinguished_name: Option<String>,

    /// The group's domain
    #[serde(rename = "GroupDomain", skip_serializing_if = "Option::is_none")]
    pub group_domain: Option<String>,

    /// The group name
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The group's distinguished name
    #[serde(rename = "GroupDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub group_distinguished_name: Option<String>,
}

impl ForeignUser {
    /// Creates a new ForeignUser
    pub fn new(
        user_domain: Option<String>,
        user_name: Option<String>,
        user_distinguished_name: Option<String>,
        group_domain: Option<String>,
        group_name: Option<String>,
        group_distinguished_name: Option<String>,
    ) -> Self {
        Self {
            user_domain,
            user_name,
            user_distinguished_name,
            group_domain,
            group_name,
            group_distinguished_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_user_new() {
        let user = ForeignUser::new(
            Some("EXTERNAL".to_string()),
            Some("external.user".to_string()),
            Some("CN=external.user,CN=Users,DC=external,DC=com".to_string()),
            Some("DOMAIN".to_string()),
            Some("Domain Admins".to_string()),
            Some("CN=Domain Admins,CN=Users,DC=domain,DC=com".to_string()),
        );
        assert_eq!(user.user_domain, Some("EXTERNAL".to_string()));
        assert_eq!(user.group_domain, Some("DOMAIN".to_string()));
    }

    #[test]
    fn test_foreign_user_serde() {
        let user = ForeignUser::new(
            Some("PARTNER".to_string()),
            Some("partner.admin".to_string()),
            None,
            Some("CORP".to_string()),
            Some("IT Admins".to_string()),
            None,
        );

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("UserDomain"));
        assert!(json.contains("GroupDomain"));

        let deserialized: ForeignUser = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }
}
