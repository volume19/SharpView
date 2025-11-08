//! Method type enumeration for querying strategies

use serde::{Deserialize, Serialize};

/// Method type for enumeration operations
///
/// Specifies which underlying API or protocol to use for queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MethodType {
    /// Use native Windows API (NetAPI32, etc.)
    #[serde(rename = "API")]
    Api,

    /// Use WinNT ADSI provider
    WinNT,
}

impl Default for MethodType {
    fn default() -> Self {
        Self::Api
    }
}

impl MethodType {
    /// Returns the method name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            MethodType::Api => "API",
            MethodType::WinNT => "WinNT",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_type_variants() {
        let methods = vec![MethodType::Api, MethodType::WinNT];
        assert_eq!(methods.len(), 2);
    }

    #[test]
    fn test_method_type_default() {
        assert_eq!(MethodType::default(), MethodType::Api);
    }

    #[test]
    fn test_method_type_as_str() {
        assert_eq!(MethodType::Api.as_str(), "API");
        assert_eq!(MethodType::WinNT.as_str(), "WinNT");
    }

    #[test]
    fn test_method_type_equality() {
        assert_eq!(MethodType::Api, MethodType::Api);
        assert_ne!(MethodType::Api, MethodType::WinNT);
    }

    #[test]
    fn test_method_type_serde() {
        let method = MethodType::Api;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"API\"");

        let deserialized: MethodType = serde_json::from_str(&json).unwrap();
        assert_eq!(method, deserialized);
    }

    #[test]
    fn test_method_type_winnt_serde() {
        let method = MethodType::WinNT;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"WinNT\"");

        let deserialized: MethodType = serde_json::from_str(&json).unwrap();
        assert_eq!(method, deserialized);
    }
}
