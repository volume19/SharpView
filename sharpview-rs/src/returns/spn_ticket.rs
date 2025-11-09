//! Service Principal Name ticket information

use serde::{Deserialize, Serialize};

/// SPN Ticket
///
/// Represents a Kerberos service ticket for a Service Principal Name (SPN).
/// Used for Kerberoasting attacks where service account credentials are extracted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpnTicket {
    /// The ticket bytes as a hex stream
    #[serde(rename = "TicketByteHexStream", skip_serializing_if = "Option::is_none")]
    pub ticket_byte_hex_stream: Option<String>,

    /// The hash of the ticket
    #[serde(rename = "Hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    /// The SAM account name
    #[serde(rename = "SamAccountName", skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,

    /// The distinguished name
    #[serde(rename = "DistinguishedName", skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,

    /// The service principal name
    #[serde(rename = "ServicePrincipalName", skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
}

impl SpnTicket {
    /// Creates a new SpnTicket
    pub fn new(
        ticket_byte_hex_stream: Option<String>,
        hash: Option<String>,
        sam_account_name: Option<String>,
        distinguished_name: Option<String>,
        service_principal_name: Option<String>,
    ) -> Self {
        Self {
            ticket_byte_hex_stream,
            hash,
            sam_account_name,
            distinguished_name,
            service_principal_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spn_ticket_new() {
        let ticket = SpnTicket::new(
            Some("6082...".to_string()),
            Some("$krb5tgs$23$*...".to_string()),
            Some("sqlservice".to_string()),
            Some("CN=sqlservice,CN=Users,DC=domain,DC=com".to_string()),
            Some("MSSQLSvc/sql01.domain.com:1433".to_string()),
        );
        assert_eq!(ticket.sam_account_name, Some("sqlservice".to_string()));
        assert_eq!(ticket.service_principal_name, Some("MSSQLSvc/sql01.domain.com:1433".to_string()));
    }

    #[test]
    fn test_spn_ticket_serde() {
        let ticket = SpnTicket::new(
            None,
            Some("$krb5tgs$23$*test...".to_string()),
            Some("svc_account".to_string()),
            None,
            Some("HTTP/web01.corp.com".to_string()),
        );

        let json = serde_json::to_string(&ticket).unwrap();
        assert!(json.contains("Hash"));
        assert!(json.contains("SamAccountName"));

        let deserialized: SpnTicket = serde_json::from_str(&json).unwrap();
        assert_eq!(ticket, deserialized);
    }
}
