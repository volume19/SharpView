//! Windows event trait
//!
//! Represents common properties for Windows security events.

use chrono::{DateTime, Utc};

/// Windows event interface
///
/// Trait for types that represent Windows security events.
/// Provides common event properties like computer name, timestamp, and event ID.
pub trait WinEvent {
    /// Gets the computer name where the event occurred
    fn computer_name(&self) -> Option<&str>;

    /// Gets the event creation timestamp
    fn time_created(&self) -> Option<DateTime<Utc>>;

    /// Gets the Windows event ID
    fn event_id(&self) -> i32;

    /// Sets the computer name
    fn set_computer_name(&mut self, name: String);

    /// Sets the event creation timestamp
    fn set_time_created(&mut self, time: Option<DateTime<Utc>>);

    /// Sets the event ID
    fn set_event_id(&mut self, id: i32);
}
