//! Windows Terminal Services connection state enumeration

use serde::{Deserialize, Serialize};

/// WTS connection state
///
/// Specifies the connection state of a Windows Terminal Services session.
/// Maps to WTS_CONNECTSTATE_CLASS from the Windows API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum WtsConnectState {
    /// Session is active (0)
    #[serde(rename = "WTSActive")]
    Active = 0,

    /// Session is connected (1)
    #[serde(rename = "WTSConnected")]
    Connected = 1,

    /// Session is in connect query state (2)
    #[serde(rename = "WTSConnectQuery")]
    ConnectQuery = 2,

    /// Session is being shadowed (3)
    #[serde(rename = "WTSShadow")]
    Shadow = 3,

    /// Session is disconnected (4)
    #[serde(rename = "WTSDisconnected")]
    Disconnected = 4,

    /// Session is idle (5)
    #[serde(rename = "WTSIdle")]
    Idle = 5,

    /// Session is listening (6)
    #[serde(rename = "WTSListen")]
    Listen = 6,

    /// Session is being reset (7)
    #[serde(rename = "WTSReset")]
    Reset = 7,

    /// Session is down (8)
    #[serde(rename = "WTSDown")]
    Down = 8,

    /// Session is initializing (9)
    #[serde(rename = "WTSInit")]
    Init = 9,
}

impl WtsConnectState {
    /// Creates a WtsConnectState from a raw u32 value
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Active),
            1 => Some(Self::Connected),
            2 => Some(Self::ConnectQuery),
            3 => Some(Self::Shadow),
            4 => Some(Self::Disconnected),
            5 => Some(Self::Idle),
            6 => Some(Self::Listen),
            7 => Some(Self::Reset),
            8 => Some(Self::Down),
            9 => Some(Self::Init),
            _ => None,
        }
    }

    /// Checks if the session is connected (active or connected state)
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Active | Self::Connected)
    }

    /// Checks if the session is disconnected or down
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Disconnected | Self::Down)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wts_connect_state_values() {
        assert_eq!(WtsConnectState::Active as u32, 0);
        assert_eq!(WtsConnectState::Connected as u32, 1);
        assert_eq!(WtsConnectState::Disconnected as u32, 4);
        assert_eq!(WtsConnectState::Init as u32, 9);
    }

    #[test]
    fn test_wts_connect_state_from_value() {
        assert_eq!(WtsConnectState::from_value(0), Some(WtsConnectState::Active));
        assert_eq!(WtsConnectState::from_value(4), Some(WtsConnectState::Disconnected));
        assert_eq!(WtsConnectState::from_value(99), None);
    }

    #[test]
    fn test_wts_connect_state_is_connected() {
        assert!(WtsConnectState::Active.is_connected());
        assert!(WtsConnectState::Connected.is_connected());
        assert!(!WtsConnectState::Disconnected.is_connected());
        assert!(!WtsConnectState::Idle.is_connected());
    }

    #[test]
    fn test_wts_connect_state_is_inactive() {
        assert!(WtsConnectState::Disconnected.is_inactive());
        assert!(WtsConnectState::Down.is_inactive());
        assert!(!WtsConnectState::Active.is_inactive());
        assert!(!WtsConnectState::Connected.is_inactive());
    }

    #[test]
    fn test_wts_connect_state_serde() {
        let state = WtsConnectState::Active;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"WTSActive\"");

        let deserialized: WtsConnectState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }
}
