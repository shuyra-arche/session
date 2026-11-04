//! Host integration coverage for the aggregate registration itself: ordered
//! membership of the provider declarations and the prefixed image coordinates.
//! These are not the runtime acceptance gates recorded in `ACCEPTANCE.md`.

use session_cluster::{IMAGE_BUILDS, UNITS, account, login, terminal};
use unit::{Dependency, Ending, Ready};

#[test]
fn units_are_registered_in_order_by_identity() {
    assert_eq!(UNITS.len(), 3);
    assert!(core::ptr::eq(UNITS[0], &account::UNIT));
    assert!(core::ptr::eq(UNITS[1], &terminal::UNIT));
    assert!(core::ptr::eq(UNITS[2], &login::PROGRAM));
    assert_eq!(
        UNITS.iter().map(|unit| unit.name).collect::<Vec<_>>(),
        ["account", "terminal", "login"]
    );
}

#[test]
fn image_builds_are_ordered_unique_and_provider_owned() {
    assert_eq!(IMAGE_BUILDS.len(), 3);
    assert_eq!(
        IMAGE_BUILDS.iter().map(|build| build.image).collect::<Vec<_>>(),
        ["account", "terminal", "login"]
    );

    let hosts = IMAGE_BUILDS
        .iter()
        .map(|build| (build.package, build.bin))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(hosts.len(), 3);

    assert_eq!(IMAGE_BUILDS[0].manifest, "units/account/server/Cargo.toml");
    assert_eq!(IMAGE_BUILDS[1].manifest, "units/terminal/server/Cargo.toml");
    assert_eq!(IMAGE_BUILDS[2].manifest, "units/login/server/Cargo.toml");

    for (cluster, provider) in [
        (&IMAGE_BUILDS[0], &account::BUILD),
        (&IMAGE_BUILDS[1], &terminal::BUILD),
        (&IMAGE_BUILDS[2], &login::BUILD),
    ] {
        assert_eq!(cluster.image, provider.image);
        assert_eq!(cluster.package, provider.package);
        assert_eq!(cluster.bin, provider.bin);
        assert_eq!(cluster.features, provider.features);
        assert_eq!(cluster.target, provider.target);
        assert_eq!(cluster.profile, provider.profile);
        assert_eq!(cluster.default_features, provider.default_features);
    }

    assert_eq!(
        IMAGE_BUILDS[2].manifest,
        format!("units/login/{}", login::BUILD.manifest)
    );
}

#[test]
fn terminal_and_login_keep_their_provider_lifecycles() {
    let terminal_control = terminal::UNIT.control;
    assert!(matches!(terminal_control.ending, Some(Ending::Resident)));
    match terminal_control.ready {
        Ready::Signal(mark) => assert_eq!(mark, unit::READY_MARK),
        other => panic!("terminal ready changed: {other:?}"),
    }
    assert_eq!(
        terminal_control.after,
        Some(
            &[
                Dependency::Bootstrap(unit::Bootstrap::Operator),
                Dependency::Unit("uart"),
            ][..]
        )
    );

    let login_control = login::PROGRAM.control;
    assert!(matches!(login_control.ending, Some(Ending::Told)));
    assert!(matches!(login_control.ready, Ready::OnSpawn));
    assert_eq!(
        login_control.after,
        Some(
            &[
                Dependency::Bootstrap(unit::Bootstrap::Operator),
                Dependency::Unit("terminal"),
                Dependency::Unit("account"),
                Dependency::Target(unit::SCENE),
            ][..]
        )
    );
}
