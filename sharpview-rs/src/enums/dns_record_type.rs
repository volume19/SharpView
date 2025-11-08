//! DNS record type enumeration
//!
//! DNS Resource Record (RR) types for DNS queries.

use serde::{Deserialize, Serialize};

/// DNS record types
///
/// Standard DNS resource record types as defined in RFC specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::upper_case_acronyms)]
pub enum DnsRecordType {
    /// IPv4 address record (RFC 1035)
    A,
    /// Name server record (RFC 1035)
    NS,
    /// Mail destination record (RFC 883 - obsolete)
    MD,
    /// Mail forwarder record (RFC 883 - obsolete)
    MF,
    /// Canonical name record (alias) (RFC 1035)
    CNAME,
    /// Start of authority record (RFC 1035)
    SOA,
    /// Mailbox record (RFC 883)
    MB,
    /// Mail group member record (RFC 883)
    MG,
    /// Mail rename record (RFC 883)
    MR,
    /// Null record (RFC 1035)
    NULL,
    /// Well-known service record (RFC 1035)
    WKS,
    /// Pointer record (RFC 1035)
    PTR,
    /// Host information record (RFC 1035)
    HINFO,
    /// Mailbox information record (RFC 1035)
    MINFO,
    /// Mail exchange record (RFC 1035)
    MX,
    /// Text record (RFC 1035)
    TXT,
    /// Responsible person record (RFC 1183)
    RP,
    /// AFS database record (RFC 1183)
    AFSDB,
    /// X.25 PSDN address record (RFC 1183)
    X25,
    /// ISDN address record (RFC 1183)
    ISDN,
    /// Route through record (RFC 1183)
    RT,
    /// NSAP address record (RFC 1706)
    NSAP,
    /// NSAP pointer record (RFC 1348)
    NSAPPTR,
    /// Signature record (RFC 2535)
    SIG,
    /// Key record (RFC 2535)
    KEY,
    /// Pointer to X.400 mapping (RFC 2163)
    PX,
    /// Geographical position record (RFC 1712)
    GPOS,
    /// IPv6 address record (RFC 3596)
    AAAA,
    /// Location record (RFC 1876)
    LOC,
    /// Next domain record (RFC 3755)
    NXT,
    /// Endpoint identifier (RFC not assigned)
    EID,
    /// Nimrod locator (RFC not assigned)
    NIMLOC,
    /// Service locator record (RFC 2782)
    SRV,
    /// ATM address record
    ATMA,
    /// Naming authority pointer record (RFC 2915)
    NAPTR,
    /// Key exchanger record (RFC 2230)
    KX,
    /// Certificate record (RFC 4398)
    CERT,
    /// IPv6 address record (RFC 2874 - obsolete)
    A6,
    /// Delegation name record (RFC 6672)
    DNAME,
    /// Sink record
    SINK,
    /// Option record (RFC 6891)
    OPT,
    /// Address prefix list (RFC 3123)
    APL,
    /// Delegation signer record (RFC 4034)
    DS,
    /// SSH fingerprint record (RFC 4255)
    SSHFP,
    /// IPsec key record (RFC 4025)
    IPSECKEY,
    /// DNSSEC signature record (RFC 4034)
    RRSIG,
    /// Next secure record (RFC 4034)
    NSEC,
    /// DNSSEC public key record (RFC 4034)
    DNSKEY,
    /// DHCP identifier record (RFC 4701)
    DHCID,
    /// Next secure record v3 (RFC 5155)
    NSEC3,
    /// NSEC3 parameters record (RFC 5155)
    NSEC3PARAM,
    /// Host identity protocol record (RFC 8005)
    HIP,
    /// Sender policy framework record (RFC 7208)
    SPF,
    /// User information record
    UINFO,
    /// User identifier record
    UID,
    /// Group identifier record
    GID,
    /// Unspecified record
    UNSPEC,
    /// Any/all records query (RFC 1035)
    ANY,
    /// Trust authorities record (proposed)
    TA,
    /// DNSSEC lookaside validation record (RFC 4431)
    DLV,
    /// Unknown record type
    UNKNOWN,
}

impl DnsRecordType {
    /// Returns the record type name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            DnsRecordType::A => "A",
            DnsRecordType::NS => "NS",
            DnsRecordType::CNAME => "CNAME",
            DnsRecordType::SOA => "SOA",
            DnsRecordType::PTR => "PTR",
            DnsRecordType::MX => "MX",
            DnsRecordType::TXT => "TXT",
            DnsRecordType::AAAA => "AAAA",
            DnsRecordType::SRV => "SRV",
            DnsRecordType::ANY => "ANY",
            _ => "UNKNOWN",
        }
    }

    /// Returns true if this is a commonly used record type
    pub fn is_common(&self) -> bool {
        matches!(
            self,
            DnsRecordType::A
                | DnsRecordType::NS
                | DnsRecordType::CNAME
                | DnsRecordType::SOA
                | DnsRecordType::PTR
                | DnsRecordType::MX
                | DnsRecordType::TXT
                | DnsRecordType::AAAA
                | DnsRecordType::SRV
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_record_type_variants() {
        // Test a few key record types
        let types = vec![
            DnsRecordType::A,
            DnsRecordType::AAAA,
            DnsRecordType::CNAME,
            DnsRecordType::MX,
            DnsRecordType::TXT,
        ];
        assert_eq!(types.len(), 5);
    }

    #[test]
    fn test_dns_record_type_as_str() {
        assert_eq!(DnsRecordType::A.as_str(), "A");
        assert_eq!(DnsRecordType::AAAA.as_str(), "AAAA");
        assert_eq!(DnsRecordType::MX.as_str(), "MX");
    }

    #[test]
    fn test_dns_record_type_is_common() {
        assert!(DnsRecordType::A.is_common());
        assert!(DnsRecordType::AAAA.is_common());
        assert!(DnsRecordType::MX.is_common());
        assert!(!DnsRecordType::AFSDB.is_common());
        assert!(!DnsRecordType::UNKNOWN.is_common());
    }

    #[test]
    fn test_dns_record_type_serde() {
        let record_type = DnsRecordType::A;
        let json = serde_json::to_string(&record_type).unwrap();
        assert_eq!(json, "\"A\"");

        let deserialized: DnsRecordType = serde_json::from_str(&json).unwrap();
        assert_eq!(record_type, deserialized);
    }

    #[test]
    fn test_dns_record_type_all_serialize() {
        // Test serialization of various types
        let test_cases = vec![
            (DnsRecordType::A, "\"A\""),
            (DnsRecordType::AAAA, "\"AAAA\""),
            (DnsRecordType::CNAME, "\"CNAME\""),
            (DnsRecordType::SRV, "\"SRV\""),
            (DnsRecordType::TXT, "\"TXT\""),
        ];

        for (record_type, expected_json) in test_cases {
            let json = serde_json::to_string(&record_type).unwrap();
            assert_eq!(json, expected_json);
        }
    }
}
