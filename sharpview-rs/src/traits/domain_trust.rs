//! Domain trust trait
//!
//! Represents common properties for domain trust relationships.

/// Domain trust interface
///
/// Trait for types that represent domain trust relationships.
/// Implemented by both LDAP-based and API-based trust types.
pub trait DomainTrust {
    /// Gets the source domain name
    fn source_name(&self) -> Option<&str>;

    /// Gets the target domain name
    fn target_name(&self) -> Option<&str>;

    /// Sets the source domain name
    fn set_source_name(&mut self, name: String);

    /// Sets the target domain name
    fn set_target_name(&mut self, name: String);
}
