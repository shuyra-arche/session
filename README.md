# Session Cluster

This repository is the **Session Cluster aggregate** only. It owns the explicit
member/build interface and joint acceptance contract; it does not own Unit Core
or copy Unit implementation.

## Fixed Unit submodules

- `units/account` at public Account
  `7dc701facdbe6368fbc09f7429920ad49dacdfca`.
- `units/terminal` at accepted public Terminal
  `f07a0954f65472e9a16e878e52c5f1d34146bc64`.

`cluster/session-cluster` is a pure `no_std` package. It reexports the two Unit
declarations and exposes explicit `UNITS` and `IMAGE_BUILDS` slices with
repository-relative manifests `units/account/server/Cargo.toml` and
`units/terminal/server/Cargo.toml`. It has no server/client/runtime dependency,
no global registry and no new Cluster implementation type.

Accepted external identities stay fixed: Terminal API
`c447c0761835339a39efdf2ddd8101ebf80db15a`; System API
`9e5d63ca536a176583cf1b0054b4ecf7ddd88257` and System client/support
`5a792ff2d39901a970925b1a6d63206adf6c16d8`; Kernel
`3aa0a155f825fbf377fc8d04009472ca67cbbf8c`; System runtime
`9001af65f97501dfa490c9397c61d593c871136d`.

**Login** remains Source-owned and pending. The actual combined Source
composition, selected Deployment and AccountRig stay Source/Vanllia-owned. This
is not a complete Session or migration claim.

Root owns Cargo locks, provenance, nested gitlink validation, native/RV runs,
publication and the recursive clean-checkout proof. Joint acceptance remains
pending; see `ACCEPTANCE.md`. Original Source base remains
`293c15fc97a9d9a4295810d10fe804e4cdec63c4`.
