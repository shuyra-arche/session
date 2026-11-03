# Session Cluster

This repository introduces the Session Cluster with its first migrated Unit,
**Account**. It is not a claim that the full Session roster has moved.

## Current members

- `units/account`: Account API, client, server and pure unit declaration, plus
  Unit-owned host tests.
- `cluster`: `session-cluster`, a `no_std` package exposing the explicit
  `UNITS` and `IMAGE_BUILDS` slices for the current Account slice. It contains no
  global registry type and no server/client runtime dependency.

## Pending and external

- **Terminal** remains pending relocation. Its accepted public API identity and
  `c447c0761835339a39efdf2ddd8101ebf80db15a` dependency stay fixed.
- **Login** remains Source-owned; no login runtime is copied here.
- The actual combined Session composition list, selected Deployment and
  integration acceptance remain owned by Source/Vanllia. No Cluster Rust type
  duplicates the `unit` schema.

## Source and builds

Account was extracted from Source
`293c15fc97a9d9a4295810d10fe804e4cdec63c4`. The Account declaration `BUILD`
names `units/account/server/Cargo.toml`, package `account-server`, binary
`account`. The root workspace builds `cluster` and the Account Unit; host tests
under `units/account/tests` remain separate workspaces. Root owns Cargo locks,
native checks, publication and Source intake.

See `PROVENANCE.md` for the checkpoint map and license status.
