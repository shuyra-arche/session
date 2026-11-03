#![no_std]
//! Explicit Session Cluster members for the current Account and fixed Terminal slices.

pub use account_declaration as account;
pub use terminal_declaration as terminal;

pub static UNITS: &[&unit::UnitFile] = &[&account::UNIT, &terminal::UNIT];

const ACCOUNT_BUILD: unit::ImageBuild = unit::ImageBuild {
    image: account::BUILD.image,
    manifest: "units/account/server/Cargo.toml",
    package: account::BUILD.package,
    bin: account::BUILD.bin,
    features: account::BUILD.features,
    target: account::BUILD.target,
    profile: account::BUILD.profile,
    default_features: account::BUILD.default_features,
};

const TERMINAL_BUILD: unit::ImageBuild = unit::ImageBuild {
    image: terminal::BUILD.image,
    manifest: "units/terminal/server/Cargo.toml",
    package: terminal::BUILD.package,
    bin: terminal::BUILD.bin,
    features: terminal::BUILD.features,
    target: terminal::BUILD.target,
    profile: terminal::BUILD.profile,
    default_features: terminal::BUILD.default_features,
};

pub static IMAGE_BUILDS: &[unit::ImageBuild] = &[ACCOUNT_BUILD, TERMINAL_BUILD];
