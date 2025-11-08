//! Computer site information

use serde::{Deserialize, Serialize};

/// Computer site
///
/// Represents information about the Active Directory site of a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputerSite {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The IP address
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// The site name
    #[serde(rename = "SiteName", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
}

impl ComputerSite {
    /// Creates a new ComputerSite
    pub fn new(
        computer_name: Option<String>,
        ip_address: Option<String>,
        site_name: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            ip_address,
            site_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_site_new() {
        let site = ComputerSite::new(
            Some("DC01".to_string()),
            Some("192.168.1.10".to_string()),
            Some("Default-First-Site-Name".to_string()),
        );
        assert_eq!(site.computer_name, Some("DC01".to_string()));
        assert_eq!(site.ip_address, Some("192.168.1.10".to_string()));
        assert_eq!(site.site_name, Some("Default-First-Site-Name".to_string()));
    }

    #[test]
    fn test_computer_site_serde() {
        let site = ComputerSite::new(
            Some("WEB01".to_string()),
            Some("10.0.0.50".to_string()),
            Some("HQ-Site".to_string()),
        );

        let json = serde_json::to_string(&site).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("IPAddress"));
        assert!(json.contains("SiteName"));

        let deserialized: ComputerSite = serde_json::from_str(&json).unwrap();
        assert_eq!(site, deserialized);
    }
}
