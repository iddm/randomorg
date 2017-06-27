//! This module gives the build and version information about the library.
//!
//! # Usage
//!
//! ```rust,no_run
//! use randomorg::version::*;
//! 
//! println!("Crate info:\n\tVersion: {}\n\tAuthors: {}\n\tName: {}\n\tHome page:
//! {}\n\tDescription: {}",
//!     CRATE_VERSION,
//!     CRATE_AUTHORS,
//!     CRATE_NAME,
//!     CRATE_HOMEPAGE,
//!     CRATE_DESCRIPTION);
//!     
//! println!("Build info:\n\tBuild date: {}\n\tCommit: {} of {}\n\tTarget: {}\n\tGit version: {}
//! \n\tFeatures: {}\n\tProfile: {}",
//!     version::short_now(),
//!     version::short_sha(),
//!     version::commit_date(),
//!     version::target(),
//!     version::semver(),
//!     build::features(),
//!     build::profile());
//! ```

/// Contains version information about the library, mostly got from git.
pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}

/// Contains build information about the library, taken from build environment.
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

/// Crate `version` field from library's `Cargo.toml`
pub const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// Crate `authors` field from library's `Cargo.toml`
pub const CRATE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
/// Crate `package` field from library's `Cargo.toml`
pub const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
/// Crate `homepage` field from library's `Cargo.toml`
pub const CRATE_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
/// Crate `description` field from library's `Cargo.toml`
pub const CRATE_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
