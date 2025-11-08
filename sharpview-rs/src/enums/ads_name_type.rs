//! Active Directory name format enumeration
//!
//! Maps to ADS_NAME_TYPE_ENUM from ActiveDs COM library.

use serde::{Deserialize, Serialize};

/// AD name format types
///
/// Specifies the format of names in Active Directory for name translation operations.
/// Maps to ADS_NAME_TYPE_ENUM COM values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AdsNameType {
    /// Distinguished Name format
    /// Example: CN=Phineas Flynn,OU=Engineers,DC=fabrikam,DC=com
    #[serde(rename = "DN")]
    Dn = 1,

    /// Canonical name format
    /// Example: fabrikam.com/Engineers/Phineas Flynn
    Canonical = 2,

    /// Windows NT 4.0 account name format
    /// Example: fabrikam\pflynn
    #[serde(rename = "NT4")]
    Nt4 = 3,

    /// Display name format
    /// Example: pflynn
    Display = 4,

    /// Domain simple name format
    /// Example: pflynn@fabrikam.com
    DomainSimple = 5,

    /// Enterprise simple name format
    /// Example: pflynn@fabrikam.com
    EnterpriseSimple = 6,

    /// GUID format
    /// Example: {95ee9fff-3436-11d1-b2b0-d15ae3ac8436}
    #[serde(rename = "GUID")]
    Guid = 7,

    /// Unknown type - let server do translation
    Unknown = 8,

    /// User Principal Name format
    /// Example: pflynn@fabrikam.com
    #[serde(rename = "UPN")]
    Upn = 9,

    /// Canonical extended format
    /// Example: fabrikam.com/Users/Phineas Flynn
    CanonicalEx = 10,

    /// Service Principal Name format
    /// Example: HTTP/kairomac.contoso.com
    #[serde(rename = "SPN")]
    Spn = 11,

    /// SID or SID history name format
    /// Example: S-1-5-21-12986231-600641547-709122288-57999
    #[serde(rename = "SID")]
    Sid = 12,
}

impl AdsNameType {
    /// Returns the raw u32 value for COM interop
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ads_name_type_values() {
        assert_eq!(AdsNameType::Dn as u32, 1);
        assert_eq!(AdsNameType::Canonical as u32, 2);
        assert_eq!(AdsNameType::Upn as u32, 9);
        assert_eq!(AdsNameType::Sid as u32, 12);
    }

    #[test]
    fn test_ads_name_type_serde() {
        let name_type = AdsNameType::Upn;
        let json = serde_json::to_string(&name_type).unwrap();
        assert_eq!(json, "\"UPN\"");

        let deserialized: AdsNameType = serde_json::from_str(&json).unwrap();
        assert_eq!(name_type, deserialized);
    }

    #[test]
    fn test_ads_name_type_all_variants() {
        let test_cases = vec![
            (AdsNameType::Dn, "\"DN\"", 1),
            (AdsNameType::Nt4, "\"NT4\"", 3),
            (AdsNameType::Guid, "\"GUID\"", 7),
            (AdsNameType::Spn, "\"SPN\"", 11),
        ];

        for (name_type, expected_json, expected_value) in test_cases {
            let json = serde_json::to_string(&name_type).unwrap();
            assert_eq!(json, expected_json);
            assert_eq!(name_type.as_u32(), expected_value);

            let deserialized: AdsNameType = serde_json::from_str(&json).unwrap();
            assert_eq!(name_type, deserialized);
        }
    }
}
