//! SPN ticket output format enumeration for Kerberoasting

use serde::{Deserialize, Serialize};

/// SPN ticket output format
///
/// Specifies the format for Kerberos ticket output during Kerberoasting operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SpnTicketFormat {
    /// John the Ripper format
    John,

    /// Hashcat format
    Hashcat,
}

impl Default for SpnTicketFormat {
    fn default() -> Self {
        Self::Hashcat
    }
}

impl SpnTicketFormat {
    /// Returns the format name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            SpnTicketFormat::John => "John",
            SpnTicketFormat::Hashcat => "Hashcat",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spn_ticket_format_default() {
        assert_eq!(SpnTicketFormat::default(), SpnTicketFormat::Hashcat);
    }

    #[test]
    fn test_spn_ticket_format_as_str() {
        assert_eq!(SpnTicketFormat::John.as_str(), "John");
        assert_eq!(SpnTicketFormat::Hashcat.as_str(), "Hashcat");
    }

    #[test]
    fn test_spn_ticket_format_serde() {
        let format = SpnTicketFormat::Hashcat;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "\"Hashcat\"");

        let deserialized: SpnTicketFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(format, deserialized);
    }
}
