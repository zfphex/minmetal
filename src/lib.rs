#![cfg_attr(not(target_os = "macos"), allow(unused))]

#[cfg(target_os = "macos")]
mod metal;

#[cfg(target_os = "macos")]
pub use metal::*;
