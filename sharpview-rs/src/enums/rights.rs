//! Active Directory rights enumeration for ACL operations

use serde::{Deserialize, Serialize};

/// AD rights for ACL operations
///
/// Specifies the type of access rights to grant or check in Active Directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Rights {
    /// Full control (all rights)
    All,

    /// Right to reset user password
    ResetPassword,

    /// Right to modify group membership
    WriteMembers,

    /// DCSync rights (replication privileges)
    ///
    /// Allows replication of password hashes and other sensitive data.
    /// Requires DS-Replication-Get-Changes and DS-Replication-Get-Changes-All extended rights.
    #[serde(rename = "DCSync")]
    DcSync,
}

impl Rights {
    /// Returns the rights name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Rights::All => "All",
            Rights::ResetPassword => "ResetPassword",
            Rights::WriteMembers => "WriteMembers",
            Rights::DcSync => "DCSync",
        }
    }

    /// Returns true if this right is considered highly privileged
    pub fn is_privileged(&self) -> bool {
        matches!(self, Rights::All | Rights::DcSync)
    }

    /// Returns a description of the right
    pub fn description(&self) -> &'static str {
        match self {
            Rights::All => "Full control over the object",
            Rights::ResetPassword => "Ability to reset user passwords",
            Rights::WriteMembers => "Ability to modify group membership",
            Rights::DcSync => {
                "Replication privileges allowing extraction of password hashes (DCSync attack)"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rights_variants() {
        let rights = vec![
            Rights::All,
            Rights::ResetPassword,
            Rights::WriteMembers,
            Rights::DcSync,
        ];
        assert_eq!(rights.len(), 4);
    }

    #[test]
    fn test_rights_as_str() {
        assert_eq!(Rights::All.as_str(), "All");
        assert_eq!(Rights::ResetPassword.as_str(), "ResetPassword");
        assert_eq!(Rights::WriteMembers.as_str(), "WriteMembers");
        assert_eq!(Rights::DcSync.as_str(), "DCSync");
    }

    #[test]
    fn test_rights_is_privileged() {
        assert!(Rights::All.is_privileged());
        assert!(Rights::DcSync.is_privileged());
        assert!(!Rights::ResetPassword.is_privileged());
        assert!(!Rights::WriteMembers.is_privileged());
    }

    #[test]
    fn test_rights_description() {
        assert!(!Rights::All.description().is_empty());
        assert!(Rights::DcSync
            .description()
            .contains("password hashes"));
    }

    #[test]
    fn test_rights_equality() {
        assert_eq!(Rights::All, Rights::All);
        assert_ne!(Rights::All, Rights::ResetPassword);
    }

    #[test]
    fn test_rights_serde() {
        let right = Rights::DcSync;
        let json = serde_json::to_string(&right).unwrap();
        assert_eq!(json, "\"DCSync\"");

        let deserialized: Rights = serde_json::from_str(&json).unwrap();
        assert_eq!(right, deserialized);
    }

    #[test]
    fn test_all_rights_serde() {
        let test_cases = vec![
            (Rights::All, "\"All\""),
            (Rights::ResetPassword, "\"ResetPassword\""),
            (Rights::WriteMembers, "\"WriteMembers\""),
            (Rights::DcSync, "\"DCSync\""),
        ];

        for (right, expected_json) in test_cases {
            let json = serde_json::to_string(&right).unwrap();
            assert_eq!(json, expected_json);

            let deserialized: Rights = serde_json::from_str(&json).unwrap();
            assert_eq!(right, deserialized);
        }
    }
}
