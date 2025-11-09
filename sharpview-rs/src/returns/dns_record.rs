//! DNS record information

use crate::enums::DnsRecordType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// DNS record
///
/// Represents a DNS record from Active Directory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsRecord {
    /// The DNS record type
    #[serde(rename = "RecordType", skip_serializing_if = "Option::is_none")]
    pub record_type: Option<DnsRecordType>,

    /// The serial number when the record was last updated
    #[serde(rename = "UpdatedAtSerial", skip_serializing_if = "Option::is_none")]
    pub updated_at_serial: Option<u32>,

    /// Time-To-Live in seconds
    #[serde(rename = "TTL", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,

    /// Age of the record
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<u32>,

    /// Timestamp (DateTime or "[static]" string)
    #[serde(rename = "TimeStamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Value>,

    /// The record data
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// The zone name
    #[serde(rename = "ZoneName", skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

    /// The record name (lowercase in C# original)
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The distinguished name (lowercase in C# original)
    #[serde(rename = "distinguishedname", skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,

    /// The DNS record object (raw data)
    #[serde(rename = "dnsrecord", skip_serializing_if = "Option::is_none")]
    pub dnsrecord: Option<Value>,

    /// When created (lowercase in C# original)
    #[serde(rename = "whencreated", skip_serializing_if = "Option::is_none")]
    pub when_created: Option<DateTime<Utc>>,

    /// When changed (lowercase in C# original)
    #[serde(rename = "whenchanged", skip_serializing_if = "Option::is_none")]
    pub when_changed: Option<DateTime<Utc>>,
}

impl DnsRecord {
    /// Creates a new DnsRecord with all fields set to None
    pub fn new() -> Self {
        Self {
            record_type: None,
            updated_at_serial: None,
            ttl: None,
            age: None,
            timestamp: None,
            data: None,
            zone_name: None,
            name: None,
            distinguished_name: None,
            dnsrecord: None,
            when_created: None,
            when_changed: None,
        }
    }
}

impl Default for DnsRecord {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_dns_record_new() {
        let record = DnsRecord::new();
        assert_eq!(record.record_type, None);
        assert_eq!(record.data, None);
    }

    #[test]
    fn test_dns_record_with_values() {
        let mut record = DnsRecord::new();
        record.record_type = Some(DnsRecordType::A);
        record.data = Some("192.168.1.10".to_string());
        record.name = Some("server01".to_string());
        record.zone_name = Some("domain.com".to_string());
        record.ttl = Some(3600);

        assert_eq!(record.record_type, Some(DnsRecordType::A));
        assert_eq!(record.data, Some("192.168.1.10".to_string()));
        assert_eq!(record.ttl, Some(3600));
    }

    #[test]
    fn test_dns_record_timestamp_static() {
        let mut record = DnsRecord::new();
        record.timestamp = Some(json!("[static]"));
        assert_eq!(record.timestamp, Some(json!("[static]")));
    }

    #[test]
    fn test_dns_record_serde() {
        let mut record = DnsRecord::new();
        record.record_type = Some(DnsRecordType::CNAME);
        record.data = Some("alias.domain.com".to_string());
        record.name = Some("www".to_string());

        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("RecordType"));
        assert!(json.contains("Data"));

        let deserialized: DnsRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record, deserialized);
    }
}
