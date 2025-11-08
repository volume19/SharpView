//! Access Control List information

use crate::returns::ResolvedSid;
use serde::{Deserialize, Serialize};

/// Access Control List entry
///
/// Represents an ACL entry for an Active Directory object, including
/// the resolved SID information and the ACE (Access Control Entry) details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Acl {
    /// Flattened ResolvedSid fields
    #[serde(flatten)]
    pub resolved_sid: ResolvedSid,

    /// The distinguished name of the object
    #[serde(rename = "ObjectDN", skip_serializing_if = "Option::is_none")]
    pub object_dn: Option<String>,

    /// The Access Control Entry (ACE) as a string representation
    ///
    /// In the C# version, this is a GenericAce object from System.Security.AccessControl.
    /// For cross-platform compatibility, we serialize it as a string.
    #[serde(rename = "Ace", skip_serializing_if = "Option::is_none")]
    pub ace: Option<String>,

    /// The SID of the object
    #[serde(rename = "ObjectSID", skip_serializing_if = "Option::is_none")]
    pub object_sid: Option<String>,

    /// Active Directory rights as a bitfield
    ///
    /// Represents System.DirectoryServices.ActiveDirectoryRights enum values.
    /// Common values:
    /// - 0x00000001: CreateChild
    /// - 0x00000002: DeleteChild
    /// - 0x00000004: ListChildren
    /// - 0x00000010: ReadProperty
    /// - 0x00000020: WriteProperty
    /// - 0x00010000: Delete
    /// - 0x00020000: ReadControl
    /// - 0x00040000: WriteDacl
    /// - 0x00080000: WriteOwner
    /// - 0x10000000: GenericAll
    #[serde(rename = "ActiveDirectoryRights")]
    pub active_directory_rights: u32,
}

impl Acl {
    /// Creates a new ACL entry
    pub fn new() -> Self {
        Self {
            resolved_sid: ResolvedSid::new(),
            object_dn: None,
            ace: None,
            object_sid: None,
            active_directory_rights: 0,
        }
    }

    /// Creates a new ACL entry with specified values
    #[allow(clippy::too_many_arguments)]
    pub fn with_values(
        resolved_sid: ResolvedSid,
        object_dn: Option<String>,
        ace: Option<String>,
        object_sid: Option<String>,
        active_directory_rights: u32,
    ) -> Self {
        Self {
            resolved_sid,
            object_dn,
            ace,
            object_sid,
            active_directory_rights,
        }
    }
}

impl Default for Acl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acl_new() {
        let acl = Acl::new();
        assert_eq!(acl.object_dn, None);
        assert_eq!(acl.ace, None);
        assert_eq!(acl.object_sid, None);
        assert_eq!(acl.active_directory_rights, 0);
    }

    #[test]
    fn test_acl_with_values() {
        let resolved_sid = ResolvedSid::with_values(
            Some("Administrator".to_string()),
            Some("DOMAIN".to_string()),
            Some("CN=Administrator,CN=Users,DC=domain,DC=com".to_string()),
            Some("user".to_string()),
        );

        let acl = Acl::with_values(
            resolved_sid.clone(),
            Some("CN=Object,DC=domain,DC=com".to_string()),
            Some("Allow GenericAll".to_string()),
            Some("S-1-5-21-...".to_string()),
            0x10000000, // GenericAll
        );

        assert_eq!(acl.resolved_sid, resolved_sid);
        assert_eq!(acl.object_dn, Some("CN=Object,DC=domain,DC=com".to_string()));
        assert_eq!(acl.ace, Some("Allow GenericAll".to_string()));
        assert_eq!(acl.object_sid, Some("S-1-5-21-...".to_string()));
        assert_eq!(acl.active_directory_rights, 0x10000000);
    }

    #[test]
    fn test_acl_serde_with_flatten() {
        let resolved_sid = ResolvedSid::with_values(
            Some("User1".to_string()),
            Some("CORP".to_string()),
            None,
            Some("user".to_string()),
        );

        let acl = Acl::with_values(
            resolved_sid,
            Some("CN=Resource,DC=corp,DC=com".to_string()),
            Some("Allow Read".to_string()),
            Some("S-1-5-21-123".to_string()),
            0x00020000, // ReadControl
        );

        let json = serde_json::to_string(&acl).unwrap();

        // Flattened fields should appear at top level
        assert!(json.contains("IdentityReferenceName"));
        assert!(json.contains("IdentityReferenceDomain"));
        assert!(json.contains("ObjectDN"));
        assert!(json.contains("Ace"));
        assert!(json.contains("ActiveDirectoryRights"));

        let deserialized: Acl = serde_json::from_str(&json).unwrap();
        assert_eq!(acl, deserialized);
    }

    #[test]
    fn test_acl_default() {
        let acl = Acl::default();
        assert_eq!(acl, Acl::new());
    }
}
