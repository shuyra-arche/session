# Session

Session groups the Account, Terminal and Login Units for Shuyra Vanllia. It owns
their pinned Git submodules and the `session-cluster` build declarations.
Unit implementations remain in their own repositories; product packaging and
runtime acceptance belong to [Vanllia](https://github.com/shuyra-arche/vanllia).

## Layout

- `units/account`, `units/terminal`, `units/login`: provider submodules.
- `cluster/src/lib.rs`: ordered `UNITS` and `IMAGE_BUILDS` compatibility exports.
- `cluster/tests/registration.rs`: provider identity, order and build-path checks.

Gitlinks select provider trees. `cluster/Cargo.toml` and `Cargo.lock` select
Cargo dependencies. Account and Terminal still use the historical `account.git`
and `terminal.git` Cargo URLs; `.gitmodules` uses their renamed repository URLs.
Changing a URL can change Cargo identity even when source bytes match.

## Prepare and verify

Install Rust through rustup; `rust-toolchain.toml` selects `nightly-2026-09-04`.
Run from the root, with network access for the first dependency fetch:

```sh
git submodule update --init --recursive
cargo test -p session-cluster --test registration --target x86_64-unknown-linux-gnu --locked
cargo check -p session-cluster --target riscv64gc-unknown-none-elf --locked
```

Host registration checks do not run the Units. Validate actual Login startup,
Account create/inspect/cancel handover and terminal foreground lending with the
selected Vanllia deployment. Retain the original exit and reclamation criteria.

Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
