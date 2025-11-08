//! Return types for SharpView operations

pub mod acl;
pub mod admin_access;
pub mod api_domain_trust;
pub mod computer_ip_address;
pub mod ldap_domain_trust;
pub mod net_domain_trust;
pub mod resolved_sid;
pub mod session_info;
pub mod share_info;
pub mod user_location;

pub use acl::Acl;
pub use admin_access::AdminAccess;
pub use api_domain_trust::ApiDomainTrust;
pub use computer_ip_address::ComputerIpAddress;
pub use ldap_domain_trust::LdapDomainTrust;
pub use net_domain_trust::NetDomainTrust;
pub use resolved_sid::ResolvedSid;
pub use session_info::SessionInfo;
pub use share_info::ShareInfo;
pub use user_location::UserLocation;
