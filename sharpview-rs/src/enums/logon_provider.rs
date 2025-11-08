//! Windows logon provider enumeration
//!
//! Maps to LOGON32_PROVIDER_* constants from Advapi32.dll LogonUser function.

use serde::{Deserialize, Serialize};

/// Logon provider for Windows LogonUser API
///
/// Specifies the logon provider to use for authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum LogonProvider {
    /// Use the standard logon provider for the system
    ///
    /// Default is negotiate, unless domain name is NULL and username is not UPN format.
    /// In that case, default provider is NTLM.
    Default = 0,

    /// Windows NT 3.5 logon provider
    WinNT35 = 1,

    /// Windows NT 4.0 logon provider
    WinNT40 = 2,

    /// Windows 2000/NT 5.0 logon provider
    ///
    /// Required for NewCredentials logon type.
    WinNT50 = 3,
}

impl LogonProvider {
    /// Returns the raw u32 value for FFI calls
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

impl Default for LogonProvider {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logon_provider_values() {
        assert_eq!(LogonProvider::Default as u32, 0);
        assert_eq!(LogonProvider::WinNT35 as u32, 1);
        assert_eq!(LogonProvider::WinNT40 as u32, 2);
        assert_eq!(LogonProvider::WinNT50 as u32, 3);
    }

    #[test]
    fn test_logon_provider_default() {
        assert_eq!(LogonProvider::default(), LogonProvider::Default);
    }

    #[test]
    fn test_logon_provider_serde() {
        let provider = LogonProvider::WinNT50;
        let json = serde_json::to_string(&provider).unwrap();
        let deserialized: LogonProvider = serde_json::from_str(&json).unwrap();
        assert_eq!(provider, deserialized);
    }
}
