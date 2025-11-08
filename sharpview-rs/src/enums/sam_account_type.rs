//! SAM account type enumeration
//!
//! Maps to sAMAccountType attribute values in Active Directory.

use serde::{Deserialize, Serialize};

/// SAM (Security Account Manager) account type
///
/// Classifies the type of an Active Directory object based on its sAMAccountType attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum SamAccountType {
    /// Domain object
    DomainObject = 0x00000000,

    /// Security-enabled group
    GroupObject = 0x10000000,

    /// Distribution group (non-security)
    NonSecurityGroupObject = 0x10000001,

    /// Security-enabled local group (alias)
    AliasObject = 0x20000000,

    /// Distribution local group (non-security)
    NonSecurityAliasObject = 0x20000001,

    /// User account object
    UserObject = 0x30000000,

    /// Computer/machine account
    MachineAccount = 0x30000001,

    /// Interdomain trust account
    TrustAccount = 0x30000002,

    /// Application basic group
    AppBasicGroup = 0x40000000,

    /// Application query group
    AppQueryGroup = 0x40000001,

    /// Maximum account type value (sentinel)
    AccountTypeMax = 0x7fffffff,
}

impl SamAccountType {
    /// Returns the raw i32 value
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    /// Returns true if this is a user-type account
    pub fn is_user(&self) -> bool {
        matches!(
            self,
            SamAccountType::UserObject
                | SamAccountType::MachineAccount
                | SamAccountType::TrustAccount
        )
    }

    /// Returns true if this is a group-type object
    pub fn is_group(&self) -> bool {
        matches!(
            self,
            SamAccountType::GroupObject
                | SamAccountType::NonSecurityGroupObject
                | SamAccountType::AliasObject
                | SamAccountType::NonSecurityAliasObject
        )
    }

    /// Returns true if this is a security principal (security-enabled)
    pub fn is_security_enabled(&self) -> bool {
        matches!(
            self,
            SamAccountType::GroupObject
                | SamAccountType::AliasObject
                | SamAccountType::UserObject
                | SamAccountType::MachineAccount
                | SamAccountType::TrustAccount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sam_account_type_values() {
        assert_eq!(SamAccountType::DomainObject as i32, 0x00000000);
        assert_eq!(SamAccountType::GroupObject as i32, 0x10000000);
        assert_eq!(SamAccountType::UserObject as i32, 0x30000000);
        assert_eq!(SamAccountType::MachineAccount as i32, 0x30000001);
        assert_eq!(SamAccountType::AccountTypeMax as i32, 0x7fffffff);
    }

    #[test]
    fn test_sam_account_type_is_user() {
        assert!(SamAccountType::UserObject.is_user());
        assert!(SamAccountType::MachineAccount.is_user());
        assert!(SamAccountType::TrustAccount.is_user());
        assert!(!SamAccountType::GroupObject.is_user());
        assert!(!SamAccountType::DomainObject.is_user());
    }

    #[test]
    fn test_sam_account_type_is_group() {
        assert!(SamAccountType::GroupObject.is_group());
        assert!(SamAccountType::NonSecurityGroupObject.is_group());
        assert!(SamAccountType::AliasObject.is_group());
        assert!(SamAccountType::NonSecurityAliasObject.is_group());
        assert!(!SamAccountType::UserObject.is_group());
        assert!(!SamAccountType::MachineAccount.is_group());
    }

    #[test]
    fn test_sam_account_type_is_security_enabled() {
        assert!(SamAccountType::GroupObject.is_security_enabled());
        assert!(SamAccountType::UserObject.is_security_enabled());
        assert!(SamAccountType::MachineAccount.is_security_enabled());
        assert!(!SamAccountType::NonSecurityGroupObject.is_security_enabled());
        assert!(!SamAccountType::NonSecurityAliasObject.is_security_enabled());
    }

    #[test]
    fn test_sam_account_type_serde() {
        let account_type = SamAccountType::UserObject;
        let json = serde_json::to_string(&account_type).unwrap();
        let deserialized: SamAccountType = serde_json::from_str(&json).unwrap();
        assert_eq!(account_type, deserialized);
    }

    #[test]
    fn test_sam_account_type_machine_account() {
        let machine = SamAccountType::MachineAccount;
        assert!(machine.is_user());
        assert!(!machine.is_group());
        assert!(machine.is_security_enabled());
        assert_eq!(machine.as_i32(), 0x30000001);
    }

    #[test]
    fn test_sam_account_type_distribution_group() {
        let dist_group = SamAccountType::NonSecurityGroupObject;
        assert!(!dist_group.is_user());
        assert!(dist_group.is_group());
        assert!(!dist_group.is_security_enabled());
    }
}
