//! Explicit credential logon event information

use crate::traits::WinEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Explicit credential logon event
///
/// Represents a Windows event where credentials were explicitly specified (Event ID 4648).
/// Implements the WinEvent trait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplicitCredentialLogonEvent {
    /// The computer name
    #[serde(rename = "ComputerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// When the event was created
    #[serde(rename = "TimeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<DateTime<Utc>>,

    /// The event ID
    #[serde(rename = "EventId")]
    pub event_id: i32,

    /// Subject domain name
    #[serde(rename = "SubjectDomainName", skip_serializing_if = "Option::is_none")]
    pub subject_domain_name: Option<String>,

    /// Subject logon ID
    #[serde(rename = "SubjectLogonId", skip_serializing_if = "Option::is_none")]
    pub subject_logon_id: Option<String>,

    /// Subject username
    #[serde(rename = "SubjectUserName", skip_serializing_if = "Option::is_none")]
    pub subject_user_name: Option<String>,

    /// Subject user SID
    #[serde(rename = "SubjectUserSid", skip_serializing_if = "Option::is_none")]
    pub subject_user_sid: Option<String>,

    /// Target domain name
    #[serde(rename = "TargetDomainName", skip_serializing_if = "Option::is_none")]
    pub target_domain_name: Option<String>,

    /// Target logon ID
    #[serde(rename = "TargetLogonId", skip_serializing_if = "Option::is_none")]
    pub target_logon_id: Option<String>,

    /// Target logon GUID
    #[serde(rename = "TargetLogonGuid", skip_serializing_if = "Option::is_none")]
    pub target_logon_guid: Option<String>,

    /// Target username
    #[serde(rename = "TargetUserName", skip_serializing_if = "Option::is_none")]
    pub target_user_name: Option<String>,

    /// Target user SID
    #[serde(rename = "TargetUserSid", skip_serializing_if = "Option::is_none")]
    pub target_user_sid: Option<String>,

    /// Target server name
    #[serde(rename = "TargetServerName", skip_serializing_if = "Option::is_none")]
    pub target_server_name: Option<String>,

    /// Logon GUID
    #[serde(rename = "LogonGuid", skip_serializing_if = "Option::is_none")]
    pub logon_guid: Option<String>,

    /// Target information
    #[serde(rename = "TargetInfo", skip_serializing_if = "Option::is_none")]
    pub target_info: Option<String>,

    /// Process ID
    #[serde(rename = "ProcessId", skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,

    /// Process name
    #[serde(rename = "ProcessName", skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,

    /// IP address
    #[serde(rename = "IpAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// IP port
    #[serde(rename = "IpPort", skip_serializing_if = "Option::is_none")]
    pub ip_port: Option<String>,
}

impl WinEvent for ExplicitCredentialLogonEvent {
    fn computer_name(&self) -> Option<&str> {
        self.computer_name.as_deref()
    }

    fn time_created(&self) -> Option<DateTime<Utc>> {
        self.time_created
    }

    fn event_id(&self) -> i32 {
        self.event_id
    }

    fn set_computer_name(&mut self, name: String) {
        self.computer_name = Some(name);
    }

    fn set_time_created(&mut self, time: Option<DateTime<Utc>>) {
        self.time_created = time;
    }

    fn set_event_id(&mut self, id: i32) {
        self.event_id = id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_explicit_credential_logon_event_trait() {
        let mut event = ExplicitCredentialLogonEvent {
            computer_name: Some("DC01".to_string()),
            time_created: Some(Utc::now()),
            event_id: 4648,
            subject_domain_name: None,
            subject_logon_id: None,
            subject_user_name: Some("user1".to_string()),
            subject_user_sid: None,
            target_domain_name: None,
            target_logon_id: None,
            target_logon_guid: None,
            target_user_name: Some("admin".to_string()),
            target_user_sid: None,
            target_server_name: Some("DC02".to_string()),
            logon_guid: None,
            target_info: None,
            process_id: None,
            process_name: None,
            ip_address: None,
            ip_port: None,
        };

        assert_eq!(event.computer_name(), Some("DC01"));
        assert_eq!(event.event_id(), 4648);

        event.set_computer_name("WS01".to_string());
        assert_eq!(event.computer_name, Some("WS01".to_string()));
    }

    #[test]
    fn test_explicit_credential_logon_event_serde() {
        let event = ExplicitCredentialLogonEvent {
            computer_name: Some("WEB01".to_string()),
            time_created: None,
            event_id: 4648,
            subject_domain_name: Some("CORP".to_string()),
            subject_logon_id: None,
            subject_user_name: Some("testuser".to_string()),
            subject_user_sid: None,
            target_domain_name: Some("DOMAIN".to_string()),
            target_logon_id: None,
            target_logon_guid: None,
            target_user_name: Some("Administrator".to_string()),
            target_user_sid: None,
            target_server_name: Some("DC01".to_string()),
            logon_guid: None,
            target_info: Some("DC01.domain.com".to_string()),
            process_id: Some("1234".to_string()),
            process_name: Some("powershell.exe".to_string()),
            ip_address: None,
            ip_port: None,
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("EventId"));
        assert!(json.contains("TargetServerName"));

        let deserialized: ExplicitCredentialLogonEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}
