//! Sentinel desktop application entry point.
//!
//! This is the main binary for the desktop application.
//! All application logic is in `lib.rs`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    sentinel::run();
}
