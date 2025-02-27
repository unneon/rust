#![cfg_attr(bootstrap, feature(let_chains))]
#![feature(let_else)]
#![feature(once_cell)]
#![feature(rustc_private)]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]
// warn on lints, that are included in `rust-lang/rust`s bootstrap
#![warn(rust_2018_idioms, unused_lifetimes)]

extern crate rustc_lexer;

use std::path::PathBuf;

pub mod bless;
pub mod fmt;
pub mod lint;
pub mod new_lint;
pub mod serve;
pub mod setup;
pub mod update_lints;

#[cfg(not(windows))]
static CARGO_CLIPPY_EXE: &str = "cargo-clippy";
#[cfg(windows)]
static CARGO_CLIPPY_EXE: &str = "cargo-clippy.exe";

/// Returns the path to the `cargo-clippy` binary
#[must_use]
pub fn cargo_clippy_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("failed to get current executable name");
    path.set_file_name(CARGO_CLIPPY_EXE);
    path
}

/// Returns the path to the Clippy project directory
///
/// # Panics
///
/// Panics if the current directory could not be retrieved, there was an error reading any of the
/// Cargo.toml files or ancestor directory is the clippy root directory
#[must_use]
pub fn clippy_project_root() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    for path in current_dir.ancestors() {
        let result = std::fs::read_to_string(path.join("Cargo.toml"));
        if let Err(err) = &result {
            if err.kind() == std::io::ErrorKind::NotFound {
                continue;
            }
        }

        let content = result.unwrap();
        if content.contains("[package]\nname = \"clippy\"") {
            return path.to_path_buf();
        }
    }
    panic!("error: Can't determine root of project. Please run inside a Clippy working dir.");
}
