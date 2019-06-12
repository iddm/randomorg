//! This module gives the build and version information about the library.
//!
//! # Usage
//!
//! ```rust
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
//! println!("Build info:\n\tCommit: {} of {}\n\tTarget: {}\n\tGit version: {}
//! \n\tFeatures: {}\n\tProfile: {}",
//!     SHA,
//!     BUILD_TIMESTAMP,
//!     TARGET_TRIPLE,
//!     SEMVER,
//!     build::features(),
//!     build::profile());
//! ```

/// Commit SHA hash
pub const SHA: &str = env!("VERGEN_SHA");
/// Build timestamp.
pub const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
/// Built for this target triple.
pub const TARGET_TRIPLE: &str = env!("VERGEN_TARGET_TRIPLE");
/// Semantic version.
pub const SEMVER: &str = env!("VERGEN_SEMVER");

/// Contains build information about the library, taken from build environment.
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

/// Crate `version` field from library's `Cargo.toml`
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Crate `authors` field from library's `Cargo.toml`
pub const CRATE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
/// Crate `package` field from library's `Cargo.toml`
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
/// Crate `homepage` field from library's `Cargo.toml`
pub const CRATE_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
/// Crate `description` field from library's `Cargo.toml`
pub const CRATE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
