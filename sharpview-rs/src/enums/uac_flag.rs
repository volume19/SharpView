//! User Account Control (UAC) flags enumeration
//!
//! Maps to userAccountControl attribute values in Active Directory.

use serde::{Deserialize, Serialize};

/// User Account Control flags
///
/// Bitflags representing account status and restrictions in Active Directory.
/// These correspond to the userAccountControl attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum UacFlag {
    /// Logon script will be executed
    Script = 0x00000001,

    /// Account is disabled
    AccountDisable = 0x00000002,

    /// Home directory is required
    HomeDirRequired = 0x00000008,

    /// Account is locked out
    Lockout = 0x00000010,

    /// No password is required
    PasswordNotRequired = 0x00000020,

    /// User cannot change password
    PasswordCannotChange = 0x00000040,

    /// Encrypted text password allowed
    EncryptedTextPasswordAllowed = 0x00000080,

    /// Temporary duplicate account
    TempDuplicateAccount = 0x00000100,

    /// Normal user account
    NormalAccount = 0x00000200,

    /// Interdomain trust account
    InterdomainTrustAccount = 0x00000800,

    /// Workstation trust account
    WorkstationTrustAccount = 0x00001000,

    /// Server trust account
    ServerTrustAccount = 0x00002000,

    /// Password doesn't expire
    DontExpirePassword = 0x00010000,

    /// MNS logon account
    MnsLogonAccount = 0x00020000,

    /// Smart card required for logon
    SmartcardRequired = 0x00040000,

    /// Trusted for Kerberos delegation
    TrustedForDelegation = 0x00080000,

    /// Not delegated (sensitive account)
    NotDelegated = 0x00100000,

    /// Use DES key only for encryption
    UseDESKeyOnly = 0x00200000,

    /// Kerberos pre-authentication not required
    DontRequirePreauth = 0x00400000,

    /// Password expired
    PasswordExpired = 0x00800000,

    /// Trusted to authenticate for delegation (protocol transition)
    TrustedToAuthForDelegation = 0x01000000,

    /// Read-only domain controller (RODC) partial secrets account
    PartialSecretsAccount = 0x04000000,
}

impl UacFlag {
    /// Returns the raw u32 value
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Checks if a UAC value contains this flag
    pub fn is_set_in(self, uac_value: u32) -> bool {
        (uac_value & self.as_u32()) != 0
    }

    /// Returns all flags set in a UAC value
    pub fn from_value(uac_value: u32) -> Vec<UacFlag> {
        let all_flags = [
            UacFlag::Script,
            UacFlag::AccountDisable,
            UacFlag::HomeDirRequired,
            UacFlag::Lockout,
            UacFlag::PasswordNotRequired,
            UacFlag::PasswordCannotChange,
            UacFlag::EncryptedTextPasswordAllowed,
            UacFlag::TempDuplicateAccount,
            UacFlag::NormalAccount,
            UacFlag::InterdomainTrustAccount,
            UacFlag::WorkstationTrustAccount,
            UacFlag::ServerTrustAccount,
            UacFlag::DontExpirePassword,
            UacFlag::MnsLogonAccount,
            UacFlag::SmartcardRequired,
            UacFlag::TrustedForDelegation,
            UacFlag::NotDelegated,
            UacFlag::UseDESKeyOnly,
            UacFlag::DontRequirePreauth,
            UacFlag::PasswordExpired,
            UacFlag::TrustedToAuthForDelegation,
            UacFlag::PartialSecretsAccount,
        ];

        all_flags
            .iter()
            .filter(|flag| flag.is_set_in(uac_value))
            .copied()
            .collect()
    }

    /// Combines multiple flags into a single u32 value
    pub fn combine(flags: &[UacFlag]) -> u32 {
        flags.iter().fold(0, |acc, flag| acc | flag.as_u32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uac_flag_values() {
        assert_eq!(UacFlag::Script as u32, 0x1);
        assert_eq!(UacFlag::AccountDisable as u32, 0x2);
        assert_eq!(UacFlag::NormalAccount as u32, 0x200);
        assert_eq!(UacFlag::DontExpirePassword as u32, 0x10000);
        assert_eq!(UacFlag::PartialSecretsAccount as u32, 0x04000000);
    }

    #[test]
    fn test_uac_flag_is_set_in() {
        let uac_value = 0x202; // NormalAccount | AccountDisable
        assert!(UacFlag::NormalAccount.is_set_in(uac_value));
        assert!(UacFlag::AccountDisable.is_set_in(uac_value));
        assert!(!UacFlag::Script.is_set_in(uac_value));
    }

    #[test]
    fn test_uac_flag_from_value() {
        let uac_value = 0x202; // NormalAccount | AccountDisable
        let flags = UacFlag::from_value(uac_value);
        assert_eq!(flags.len(), 2);
        assert!(flags.contains(&UacFlag::NormalAccount));
        assert!(flags.contains(&UacFlag::AccountDisable));
    }

    #[test]
    fn test_uac_flag_combine() {
        let flags = vec![UacFlag::NormalAccount, UacFlag::DontExpirePassword];
        let combined = UacFlag::combine(&flags);
        assert_eq!(combined, 0x10200);
        assert!(UacFlag::NormalAccount.is_set_in(combined));
        assert!(UacFlag::DontExpirePassword.is_set_in(combined));
    }

    #[test]
    fn test_uac_flag_serde() {
        let flag = UacFlag::AccountDisable;
        let json = serde_json::to_string(&flag).unwrap();
        let deserialized: UacFlag = serde_json::from_str(&json).unwrap();
        assert_eq!(flag, deserialized);
    }

    #[test]
    fn test_typical_normal_user() {
        // Typical enabled user account: NormalAccount | DontExpirePassword
        let uac_value = 0x10200;
        let flags = UacFlag::from_value(uac_value);
        assert!(flags.contains(&UacFlag::NormalAccount));
        assert!(flags.contains(&UacFlag::DontExpirePassword));
        assert_eq!(flags.len(), 2);
    }

    #[test]
    fn test_disabled_account() {
        // Disabled normal account: NormalAccount | AccountDisable
        let uac_value = 0x202;
        let flags = UacFlag::from_value(uac_value);
        assert!(flags.contains(&UacFlag::NormalAccount));
        assert!(flags.contains(&UacFlag::AccountDisable));
    }
}
