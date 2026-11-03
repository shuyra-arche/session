#![no_std]
//! Explicit Session Cluster members for the current Account slice.

pub use account_declaration as account;

pub static UNITS: &[&unit::UnitFile] = &[&account::UNIT];

pub static IMAGE_BUILDS: &[unit::ImageBuild] = &[account::BUILD];
