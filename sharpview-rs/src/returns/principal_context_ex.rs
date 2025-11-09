//! Principal context extended information

use serde::{Deserialize, Serialize};

/// Principal context extended
///
/// Represents an extended principal context with identity information.
/// Note: In the C# version, this contains a System.DirectoryServices.AccountManagement.PrincipalContext
/// object. In Rust, we only store the identity as the Context object is platform-specific.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrincipalContextEx {
    /// The identity
    #[serde(rename = "Identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,

    /// Context information as a string representation
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl PrincipalContextEx {
    /// Creates a new PrincipalContextEx
    pub fn new(identity: Option<String>, context: Option<String>) -> Self {
        Self {
            identity,
            context,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_principal_context_ex_new() {
        let ctx = PrincipalContextEx::new(
            Some("DOMAIN\\user".to_string()),
            Some("Domain: DOMAIN.COM".to_string()),
        );
        assert_eq!(ctx.identity, Some("DOMAIN\\user".to_string()));
        assert_eq!(ctx.context, Some("Domain: DOMAIN.COM".to_string()));
    }

    #[test]
    fn test_principal_context_ex_serde() {
        let ctx = PrincipalContextEx::new(
            Some("admin".to_string()),
            None,
        );

        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains("Identity"));

        let deserialized: PrincipalContextEx = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx, deserialized);
    }
}
