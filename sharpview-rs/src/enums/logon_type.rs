//! Windows logon type enumeration
//!
//! Maps to LOGON32_LOGON_* constants from Advapi32.dll LogonUser function.

use serde::{Deserialize, Serialize};

/// Logon type for Windows LogonUser API
///
/// Specifies the type of logon operation to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum LogonType {
    /// Interactive logon for users using the computer interactively
    ///
    /// Intended for terminal server, remote shell, or similar processes.
    /// Caches logon information for disconnected operations.
    Interactive = 2,

    /// Network logon for high-performance server authentication
    ///
    /// Does not cache credentials. Suitable for plaintext password authentication.
    Network = 3,

    /// Batch logon for scheduled tasks and automated processes
    ///
    /// For processes executing on behalf of a user without direct intervention.
    /// Does not cache credentials.
    Batch = 4,

    /// Service logon for Windows services
    ///
    /// Account must have service privilege enabled.
    Service = 5,

    /// Unlock logon for workstation unlock scenarios
    ///
    /// For GINA DLLs logging on users interactively.
    /// Generates unique audit record for workstation unlock.
    Unlock = 7,

    /// Network logon with cleartext credentials preserved
    ///
    /// Preserves credentials in authentication package for server impersonation.
    /// Allows server to make connections to other network servers.
    NetworkCleartext = 8,

    /// New credentials logon for credential delegation
    ///
    /// Clones current token with new credentials for outbound connections.
    /// Same local identifier but different network credentials.
    /// Requires LOGON32_PROVIDER_WINNT50 provider.
    NewCredentials = 9,
}

impl LogonType {
    /// Returns the raw u32 value for FFI calls
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logon_type_values() {
        assert_eq!(LogonType::Interactive as u32, 2);
        assert_eq!(LogonType::Network as u32, 3);
        assert_eq!(LogonType::Batch as u32, 4);
        assert_eq!(LogonType::Service as u32, 5);
        assert_eq!(LogonType::Unlock as u32, 7);
        assert_eq!(LogonType::NetworkCleartext as u32, 8);
        assert_eq!(LogonType::NewCredentials as u32, 9);
    }

    #[test]
    fn test_logon_type_as_u32() {
        assert_eq!(LogonType::Interactive.as_u32(), 2);
        assert_eq!(LogonType::Network.as_u32(), 3);
    }

    #[test]
    fn test_logon_type_serde() {
        let logon_type = LogonType::Interactive;
        let json = serde_json::to_string(&logon_type).unwrap();
        let deserialized: LogonType = serde_json::from_str(&json).unwrap();
        assert_eq!(logon_type, deserialized);
    }
}
