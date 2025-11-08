//! Computer IP address information

use serde::{Deserialize, Serialize};

/// Computer IP address
///
/// Represents a computer name and its associated IP address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputerIpAddress {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The IP address
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl ComputerIpAddress {
    /// Creates a new ComputerIpAddress
    pub fn new(computer_name: Option<String>, ip_address: Option<String>) -> Self {
        Self {
            computer_name,
            ip_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_ip_address_new() {
        let info = ComputerIpAddress::new(
            Some("DC01".to_string()),
            Some("192.168.1.10".to_string()),
        );
        assert_eq!(info.computer_name, Some("DC01".to_string()));
        assert_eq!(info.ip_address, Some("192.168.1.10".to_string()));
    }

    #[test]
    fn test_computer_ip_address_partial() {
        let info = ComputerIpAddress::new(Some("WEB01".to_string()), None);
        assert_eq!(info.computer_name, Some("WEB01".to_string()));
        assert_eq!(info.ip_address, None);
    }

    #[test]
    fn test_computer_ip_address_serde() {
        let info = ComputerIpAddress::new(
            Some("DC01".to_string()),
            Some("10.0.0.1".to_string()),
        );
        let json = serde_json::to_string(&info).unwrap();

        assert!(json.contains("ComputerName"));
        assert!(json.contains("IPAddress"));

        let deserialized: ComputerIpAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }
}
