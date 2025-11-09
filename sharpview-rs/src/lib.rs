//! SharpView-RS: Rust port of SharpView
//!
//! Active Directory reconnaissance library for authorized security testing,
//! penetration testing, and defensive security operations.
//!
//! # Authorization Context
//!
//! This tool is designed for legitimate security testing with proper authorization:
//! - Authorized penetration testing engagements
//! - Security research in controlled environments
//! - CTF competitions and training
//! - Red team operations with management approval
//!
//! Unauthorized use is illegal and unethical.

pub mod args;
pub mod enums;
pub mod returns;
pub mod traits;

pub use args::*;
pub use enums::*;
pub use returns::*;
pub use traits::*;
