//! Logon event information

use crate::traits::WinEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Logon event
///
/// Represents a Windows logon event from the security event log.
/// Implements the WinEvent trait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogonEvent {
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

    /// Target username
    #[serde(rename = "TargetUserName", skip_serializing_if = "Option::is_none")]
    pub target_user_name: Option<String>,

    /// Target user SID
    #[serde(rename = "TargetUserSid", skip_serializing_if = "Option::is_none")]
    pub target_user_sid: Option<String>,

    /// Logon type
    #[serde(rename = "LogonType", skip_serializing_if = "Option::is_none")]
    pub logon_type: Option<String>,

    /// Logon process name
    #[serde(rename = "LogonProcessName", skip_serializing_if = "Option::is_none")]
    pub logon_process_name: Option<String>,

    /// Logon GUID
    #[serde(rename = "LogonGuid", skip_serializing_if = "Option::is_none")]
    pub logon_guid: Option<String>,

    /// Authentication package name
    #[serde(rename = "AuthenticationPackageName", skip_serializing_if = "Option::is_none")]
    pub authentication_package_name: Option<String>,

    /// Workstation name
    #[serde(rename = "WorkstationName", skip_serializing_if = "Option::is_none")]
    pub workstation_name: Option<String>,

    /// Transmitted services
    #[serde(rename = "TransmittedServices", skip_serializing_if = "Option::is_none")]
    pub transmitted_services: Option<String>,

    /// LM package name
    #[serde(rename = "LmPackageName", skip_serializing_if = "Option::is_none")]
    pub lm_package_name: Option<String>,

    /// Key length
    #[serde(rename = "KeyLength", skip_serializing_if = "Option::is_none")]
    pub key_length: Option<String>,

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

    /// Impersonation level
    #[serde(rename = "ImpersonationLevel", skip_serializing_if = "Option::is_none")]
    pub impersonation_level: Option<String>,

    /// Restricted admin mode
    #[serde(rename = "RestrictedAdminMode", skip_serializing_if = "Option::is_none")]
    pub restricted_admin_mode: Option<String>,

    /// Target outbound username
    #[serde(rename = "TargetOutboundUserName", skip_serializing_if = "Option::is_none")]
    pub target_outbound_user_name: Option<String>,

    /// Target outbound domain name
    #[serde(rename = "TargetOutboundDomainName", skip_serializing_if = "Option::is_none")]
    pub target_outbound_domain_name: Option<String>,

    /// Virtual account
    #[serde(rename = "VirtualAccount", skip_serializing_if = "Option::is_none")]
    pub virtual_account: Option<String>,

    /// Target linked logon ID
    #[serde(rename = "TargetLinkedLogonId", skip_serializing_if = "Option::is_none")]
    pub target_linked_logon_id: Option<String>,

    /// Elevated token
    #[serde(rename = "ElevatedToken", skip_serializing_if = "Option::is_none")]
    pub elevated_token: Option<String>,
}

impl WinEvent for LogonEvent {
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
    fn test_logon_event_trait() {
        let mut event = LogonEvent {
            computer_name: Some("DC01".to_string()),
            time_created: Some(Utc::now()),
            event_id: 4624,
            subject_domain_name: None,
            subject_logon_id: None,
            subject_user_name: None,
            subject_user_sid: None,
            target_domain_name: None,
            target_logon_id: None,
            target_user_name: Some("administrator".to_string()),
            target_user_sid: None,
            logon_type: Some("2".to_string()),
            logon_process_name: None,
            logon_guid: None,
            authentication_package_name: None,
            workstation_name: None,
            transmitted_services: None,
            lm_package_name: None,
            key_length: None,
            process_id: None,
            process_name: None,
            ip_address: Some("192.168.1.100".to_string()),
            ip_port: None,
            impersonation_level: None,
            restricted_admin_mode: None,
            target_outbound_user_name: None,
            target_outbound_domain_name: None,
            virtual_account: None,
            target_linked_logon_id: None,
            elevated_token: None,
        };

        assert_eq!(event.computer_name(), Some("DC01"));
        assert_eq!(event.event_id(), 4624);

        event.set_event_id(4625);
        assert_eq!(event.event_id, 4625);
    }

    #[test]
    fn test_logon_event_serde() {
        let event = LogonEvent {
            computer_name: Some("WS01".to_string()),
            time_created: None,
            event_id: 4624,
            subject_domain_name: None,
            subject_logon_id: None,
            subject_user_name: None,
            subject_user_sid: None,
            target_domain_name: Some("DOMAIN".to_string()),
            target_logon_id: None,
            target_user_name: Some("user1".to_string()),
            target_user_sid: None,
            logon_type: Some("3".to_string()),
            logon_process_name: None,
            logon_guid: None,
            authentication_package_name: None,
            workstation_name: None,
            transmitted_services: None,
            lm_package_name: None,
            key_length: None,
            process_id: None,
            process_name: None,
            ip_address: None,
            ip_port: None,
            impersonation_level: None,
            restricted_admin_mode: None,
            target_outbound_user_name: None,
            target_outbound_domain_name: None,
            virtual_account: None,
            target_linked_logon_id: None,
            elevated_token: None,
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("EventId"));
        assert!(json.contains("TargetUserName"));

        let deserialized: LogonEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}
