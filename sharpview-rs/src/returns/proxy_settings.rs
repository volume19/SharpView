//! Proxy settings information

use serde::{Deserialize, Serialize};

/// Proxy settings
///
/// Represents proxy configuration for a computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxySettings {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// The proxy server address
    #[serde(rename = "ProxyServer", skip_serializing_if = "Option::is_none")]
    pub proxy_server: Option<String>,

    /// The auto-configuration URL
    #[serde(rename = "AutoConfigURL", skip_serializing_if = "Option::is_none")]
    pub auto_config_url: Option<String>,

    /// Web Proxy Auto-Discovery (WPAD) setting
    #[serde(rename = "Wpad", skip_serializing_if = "Option::is_none")]
    pub wpad: Option<String>,
}

impl ProxySettings {
    /// Creates a new ProxySettings
    pub fn new(
        computer_name: Option<String>,
        proxy_server: Option<String>,
        auto_config_url: Option<String>,
        wpad: Option<String>,
    ) -> Self {
        Self {
            computer_name,
            proxy_server,
            auto_config_url,
            wpad,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_settings_new() {
        let settings = ProxySettings::new(
            Some("WS01".to_string()),
            Some("proxy.corp.com:8080".to_string()),
            Some("http://wpad.corp.com/wpad.dat".to_string()),
            Some("enabled".to_string()),
        );
        assert_eq!(settings.computer_name, Some("WS01".to_string()));
        assert_eq!(settings.proxy_server, Some("proxy.corp.com:8080".to_string()));
    }

    #[test]
    fn test_proxy_settings_serde() {
        let settings = ProxySettings::new(
            Some("DC01".to_string()),
            Some("10.0.0.1:3128".to_string()),
            None,
            None,
        );

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("ComputerName"));
        assert!(json.contains("ProxyServer"));

        let deserialized: ProxySettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }
}
