//! This is the library that powers the `sentry-cli` tool.  The primary
//! exported function is `main` which is directly invoked from the
//! compiled binrary that links against this library.

#![recursion_limit = "128"]

pub mod api;
pub mod commands;
pub mod config;
pub mod constants;
pub mod utils;

/// Executes the command line application and exits the process.
pub fn main() {
    utils::system::run_or_interrupt(commands::main);
}
