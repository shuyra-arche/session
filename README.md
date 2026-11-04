# Session Cluster

This repository is the **Session Cluster aggregate** only. It owns the explicit
member/build interface and joint acceptance contract; it does not own Unit Core
or copy Unit implementation.

## Fixed Unit submodules

- `units/account` at public Account
  `7dc701facdbe6368fbc09f7429920ad49dacdfca`.
- `units/terminal` at accepted public Terminal
  `f07a0954f65472e9a16e878e52c5f1d34146bc64`.
- `units/login` at independently published Login
  `ce8e9007817d7494e05c057e23a7ba907a5d5ba8`.

`cluster/session-cluster` is a pure `no_std` package. It reexports the three
provider declarations and exposes explicit `UNITS` and `IMAGE_BUILDS` slices in
the order account, terminal, login, with repository-relative manifests
`units/account/server/Cargo.toml`, `units/terminal/server/Cargo.toml` and
`units/login/server/Cargo.toml`. Every non-manifest build field comes from the
matching provider `BUILD`; the Cluster only prefixes the manifest. It has no
server/client/runtime dependency, no global registry and no new Cluster
implementation type.

The `units/account` and `units/terminal` submodule remotes are named
`session-account` and `session-terminal`, while the canonical Cargo dependency
URLs remain `account.git` and `terminal.git`. The renamed clone remotes are
distinct from those intentionally retained canonical Cargo URLs with their full
revisions, which this slice does not change; renaming a clone remote is not
treated as a Cargo source identity replacement.

Accepted external identities stay fixed: Terminal API
`c447c0761835339a39efdf2ddd8101ebf80db15a`; System API
`9e5d63ca536a176583cf1b0054b4ecf7ddd88257` and System client/support
`5a792ff2d39901a970925b1a6d63206adf6c16d8`; Kernel
`3aa0a155f825fbf377fc8d04009472ca67cbbf8c`; System runtime
`9001af65f97501dfa490c9397c61d593c871136d`.

Login is now an independently published provider registered in this aggregate:
it keeps its own declaration, body, host tests and provenance in
`units/login`, and this Cluster selects it. Registration is not composition.
Source/Vanllia composition, selected Deployment, AccountRig, real startup and
real terminal/Account acceptance remain pending; no runtime result is claimed.
This is not a complete Session or migration claim.

Root owns Cargo locks, provenance, nested gitlink validation, native/RV runs,
publication and the recursive clean-checkout proof. Joint acceptance remains
pending with all 12 runtime gates still PENDING; see `ACCEPTANCE.md`. Original
Source base remains
`293c15fc97a9d9a4295810d10fe804e4cdec63c4`.
