#![no_std]
//! Explicit Session Cluster members for the current Account, fixed Terminal and
//! independently published Login slices.

pub use account_declaration as account;
pub use login_declaration as login;
pub use terminal_declaration as terminal;

pub static UNITS: &[&unit::UnitFile] = &[&account::UNIT, &terminal::UNIT, &login::PROGRAM];

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

const LOGIN_BUILD: unit::ImageBuild = unit::ImageBuild {
    image: login::BUILD.image,
    manifest: "units/login/server/Cargo.toml",
    package: login::BUILD.package,
    bin: login::BUILD.bin,
    features: login::BUILD.features,
    target: login::BUILD.target,
    profile: login::BUILD.profile,
    default_features: login::BUILD.default_features,
};

pub static IMAGE_BUILDS: &[unit::ImageBuild] = &[ACCOUNT_BUILD, TERMINAL_BUILD, LOGIN_BUILD];
