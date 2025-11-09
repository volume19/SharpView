//! Arguments for Get-NetLocalGroupMember operations

use crate::args::NetworkCredential;
use crate::enums::MethodType;
use serde::{Deserialize, Serialize};

/// Arguments for Get-NetLocalGroupMember
///
/// Arguments for enumerating local group members on computers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgsGetNetLocalGroupMember {
    /// Computer names to query
    #[serde(rename = "ComputerName")]
    pub computer_name: Vec<String>,

    /// Group name to enumerate (default: Administrators)
    #[serde(rename = "GroupName")]
    pub group_name: String,

    /// Collection method (API or WinNT)
    #[serde(rename = "Method")]
    pub method: MethodType,

    /// Network credential
    #[serde(rename = "Credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<NetworkCredential>,
}

impl ArgsGetNetLocalGroupMember {
    pub fn new() -> Self {
        Self {
            computer_name: vec![],
            group_name: "Administrators".to_string(),
            method: MethodType::Api,
            credential: None,
        }
    }

    pub fn computer_name(mut self, names: Vec<String>) -> Self {
        self.computer_name = names;
        self
    }

    pub fn group_name(mut self, name: String) -> Self {
        self.group_name = name;
        self
    }

    pub fn method(mut self, method: MethodType) -> Self {
        self.method = method;
        self
    }

    pub fn collection_method(self, method: MethodType) -> Self {
        self.method(method)
    }

    pub fn credential(mut self, credential: NetworkCredential) -> Self {
        self.credential = Some(credential);
        self
    }
}

impl Default for ArgsGetNetLocalGroupMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_get_net_local_group_member_new() {
        let args = ArgsGetNetLocalGroupMember::new();
        assert_eq!(args.group_name, "Administrators");
        assert_eq!(args.method, MethodType::Api);
    }

    #[test]
    fn test_args_get_net_local_group_member_builder() {
        let args = ArgsGetNetLocalGroupMember::new()
            .computer_name(vec!["DC01".to_string()])
            .group_name("Remote Desktop Users".to_string())
            .method(MethodType::WinNT);

        assert_eq!(args.computer_name, vec!["DC01".to_string()]);
        assert_eq!(args.group_name, "Remote Desktop Users");
        assert_eq!(args.method, MethodType::WinNT);
    }
}
