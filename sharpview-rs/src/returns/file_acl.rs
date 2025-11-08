//! File access control list information

use crate::enums::AccessControlType;
use serde::{Deserialize, Serialize};

/// File ACL
///
/// Represents access control list (ACL) information for a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileAcl {
    /// The file path
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// File system rights as a string representation
    #[serde(rename = "FileSystemRights", skip_serializing_if = "Option::is_none")]
    pub file_system_rights: Option<String>,

    /// Identity references (can be multiple)
    #[serde(rename = "IdentityReference", skip_serializing_if = "Option::is_none")]
    pub identity_reference: Option<Vec<String>>,

    /// The SID of the identity
    #[serde(rename = "IdentitySID", skip_serializing_if = "Option::is_none")]
    pub identity_sid: Option<String>,

    /// The access control type (Allow or Deny)
    #[serde(rename = "AccessControlType")]
    pub access_control_type: AccessControlType,
}

impl FileAcl {
    /// Creates a new FileAcl
    pub fn new(
        path: Option<String>,
        file_system_rights: Option<String>,
        identity_reference: Option<Vec<String>>,
        identity_sid: Option<String>,
        access_control_type: AccessControlType,
    ) -> Self {
        Self {
            path,
            file_system_rights,
            identity_reference,
            identity_sid,
            access_control_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_acl_new() {
        let acl = FileAcl::new(
            Some("C:\\temp\\test.txt".to_string()),
            Some("FullControl".to_string()),
            Some(vec!["BUILTIN\\Administrators".to_string()]),
            Some("S-1-5-32-544".to_string()),
            AccessControlType::Allow,
        );
        assert_eq!(acl.path, Some("C:\\temp\\test.txt".to_string()));
        assert_eq!(acl.access_control_type, AccessControlType::Allow);
    }

    #[test]
    fn test_file_acl_deny() {
        let acl = FileAcl::new(
            Some("C:\\confidential\\data.txt".to_string()),
            Some("Read, Write".to_string()),
            Some(vec!["DOMAIN\\Users".to_string()]),
            Some("S-1-5-21-...-513".to_string()),
            AccessControlType::Deny,
        );
        assert_eq!(acl.access_control_type, AccessControlType::Deny);
    }

    #[test]
    fn test_file_acl_multiple_identities() {
        let acl = FileAcl::new(
            Some("C:\\shared\\file.doc".to_string()),
            Some("Read".to_string()),
            Some(vec![
                "DOMAIN\\User1".to_string(),
                "DOMAIN\\User2".to_string(),
            ]),
            None,
            AccessControlType::Allow,
        );
        assert_eq!(acl.identity_reference.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_file_acl_serde() {
        let acl = FileAcl::new(
            Some("C:\\test.txt".to_string()),
            Some("Modify".to_string()),
            Some(vec!["DOMAIN\\admin".to_string()]),
            Some("S-1-5-21-...-500".to_string()),
            AccessControlType::Allow,
        );

        let json = serde_json::to_string(&acl).unwrap();
        assert!(json.contains("Path"));
        assert!(json.contains("FileSystemRights"));
        assert!(json.contains("AccessControlType"));

        let deserialized: FileAcl = serde_json::from_str(&json).unwrap();
        assert_eq!(acl, deserialized);
    }
}
