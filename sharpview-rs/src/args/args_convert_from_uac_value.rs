//! Arguments for ConvertFrom-UACValue operations

use serde::{Deserialize, Serialize};

/// Arguments for ConvertFrom-UACValue
///
/// Converts a User Account Control (UAC) integer value to its flag names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsConvertFromUacValue {
    /// The UAC value to convert
    #[serde(rename = "Value")]
    pub value: i32,

    /// Show all flags (including unset ones)
    #[serde(rename = "ShowAll")]
    pub show_all: bool,
}

impl ArgsConvertFromUacValue {
    /// Creates a new ArgsConvertFromUacValue
    pub fn new(value: i32) -> Self {
        Self {
            value,
            show_all: false,
        }
    }

    /// Sets the UAC value (also accepts "UAC" and "useraccountcontrol" as aliases)
    pub fn value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    /// Alias for value (UAC)
    pub fn uac(self, uac: i32) -> Self {
        self.value(uac)
    }

    /// Alias for value (useraccountcontrol)
    pub fn user_account_control(self, uac: i32) -> Self {
        self.value(uac)
    }

    /// Sets show_all
    pub fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = show_all;
        self
    }
}

impl Default for ArgsConvertFromUacValue {
    fn default() -> Self {
        Self {
            value: 0,
            show_all: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_convert_from_uac_value_new() {
        let args = ArgsConvertFromUacValue::new(512);
        assert_eq!(args.value, 512);
        assert_eq!(args.show_all, false);
    }

    #[test]
    fn test_args_convert_from_uac_value_builder() {
        let args = ArgsConvertFromUacValue::new(0)
            .value(66048)
            .show_all(true);

        assert_eq!(args.value, 66048);
        assert_eq!(args.show_all, true);
    }

    #[test]
    fn test_args_convert_from_uac_value_aliases() {
        let args1 = ArgsConvertFromUacValue::default().uac(512);
        assert_eq!(args1.value, 512);

        let args2 = ArgsConvertFromUacValue::default().user_account_control(512);
        assert_eq!(args2.value, 512);
    }

    #[test]
    fn test_args_convert_from_uac_value_serde() {
        let args = ArgsConvertFromUacValue::new(512)
            .show_all(true);

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("Value"));
        assert!(json.contains("512"));

        let deserialized: ArgsConvertFromUacValue = serde_json::from_str(&json).unwrap();
        assert_eq!(args, deserialized);
    }
}
