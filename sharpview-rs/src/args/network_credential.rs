//! Network credential information

use serde::{Deserialize, Serialize};

/// Network credential
///
/// Represents authentication credentials for network operations.
/// Maps to System.Net.NetworkCredential in .NET.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCredential {
    /// Username
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    /// Password (stored as String for compatibility, should use SecureString in production)
    #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Domain name
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl NetworkCredential {
    /// Creates a new empty NetworkCredential
    pub fn new() -> Self {
        Self {
            user_name: None,
            password: None,
            domain: None,
        }
    }

    /// Creates a NetworkCredential with username and password
    pub fn with_credentials(user_name: String, password: String) -> Self {
        Self {
            user_name: Some(user_name),
            password: Some(password),
            domain: None,
        }
    }

    /// Creates a NetworkCredential with username, password, and domain
    pub fn with_domain(user_name: String, password: String, domain: String) -> Self {
        Self {
            user_name: Some(user_name),
            password: Some(password),
            domain: Some(domain),
        }
    }

    /// Sets the username
    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = Some(user_name);
    }

    /// Sets the password
    pub fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }

    /// Sets the domain
    pub fn set_domain(&mut self, domain: String) {
        self.domain = Some(domain);
    }
}

impl Default for NetworkCredential {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_credential_new() {
        let cred = NetworkCredential::new();
        assert_eq!(cred.user_name, None);
        assert_eq!(cred.password, None);
        assert_eq!(cred.domain, None);
    }

    #[test]
    fn test_network_credential_with_credentials() {
        let cred = NetworkCredential::with_credentials(
            "user1".to_string(),
            "password123".to_string(),
        );
        assert_eq!(cred.user_name, Some("user1".to_string()));
        assert_eq!(cred.password, Some("password123".to_string()));
        assert_eq!(cred.domain, None);
    }

    #[test]
    fn test_network_credential_with_domain() {
        let cred = NetworkCredential::with_domain(
            "user1".to_string(),
            "password123".to_string(),
            "CORP".to_string(),
        );
        assert_eq!(cred.user_name, Some("user1".to_string()));
        assert_eq!(cred.password, Some("password123".to_string()));
        assert_eq!(cred.domain, Some("CORP".to_string()));
    }

    #[test]
    fn test_network_credential_setters() {
        let mut cred = NetworkCredential::new();
        cred.set_user_name("admin".to_string());
        cred.set_password("secret".to_string());
        cred.set_domain("DOMAIN".to_string());

        assert_eq!(cred.user_name, Some("admin".to_string()));
        assert_eq!(cred.password, Some("secret".to_string()));
        assert_eq!(cred.domain, Some("DOMAIN".to_string()));
    }

    #[test]
    fn test_network_credential_serde() {
        let cred = NetworkCredential::with_domain(
            "testuser".to_string(),
            "testpass".to_string(),
            "TESTDOMAIN".to_string(),
        );

        let json = serde_json::to_string(&cred).unwrap();
        assert!(json.contains("UserName"));
        assert!(json.contains("testuser"));

        let deserialized: NetworkCredential = serde_json::from_str(&json).unwrap();
        assert_eq!(cred, deserialized);
    }

    #[test]
    fn test_network_credential_serde_with_none() {
        let cred = NetworkCredential::new();
        let json = serde_json::to_string(&cred).unwrap();
        
        // None values should be skipped in serialization
        assert_eq!(json, "{}");

        let deserialized: NetworkCredential = serde_json::from_str(&json).unwrap();
        assert_eq!(cred, deserialized);
    }
}
