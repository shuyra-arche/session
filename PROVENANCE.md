# Session aggregate provenance

Current repository boundary: Cluster aggregation with Unit Git submodules.
Vanllia fixes this Cluster; the Cluster fixes its Units. The previous public
Account-in-Cluster snapshot `6d27af19ee4f94de04bb644ac8fe78e8001e99e7` and all
its source maps remain in Git history. This change does not rewrite that history.

## Fixed Unit inputs

| Git submodule | Public repository | Complete revision |
| --- | --- | --- |
| units/account | https://github.com/shuyra-arche/account.git | `7dc701facdbe6368fbc09f7429920ad49dacdfca` |
| units/terminal | https://github.com/shuyra-arche/terminal.git | `f07a0954f65472e9a16e878e52c5f1d34146bc64` |
| units/login | https://github.com/shuyra-arche/session-login.git | `ce8e9007817d7494e05c057e23a7ba907a5d5ba8` |

The submodule remotes for Account and Terminal are the renamed
`session-account`/`session-terminal` repositories, while the canonical Cargo
dependencies intentionally keep the original `account.git`/`terminal.git`
URLs with the same full revisions. The renamed clone remotes are distinct from
those canonical Cargo URLs plus revisions, which this slice does not change;
a Cargo Git source identity includes both the URL and the full revision, so
remote renaming is not assumed to be Cargo-equivalent and no URL migration is
performed here. No dependency or feature change is part of this slice.

Account owns its declaration/API/client/server/tests and original Source
`293c15fc97a9d9a4295810d10fe804e4cdec63c4` provenance: eight public original
paths/hashes, original authors, unchanged production bodies/fields/codec and
preserved earlier test/import fixes. Its public clean checkout reproduced29
tracked files, eight host cases, four host libraries and actual RV debug/release
ELFs, with raw exits0 and unique runtime providers. The Unit's PROVENANCE is the
public source-map authority. The old22 in-Cluster files were removed only after
that public proof and exact old-file hash comparison; their bytes and previous
publication remain retained.

Terminal uses the already-accepted existing provider history at f07, with its
own original Source mappings/authors retained in units/terminal/PROVENANCE.md.
No Terminal source, runtime or API was changed or republished by this aggregation.
Terminal API/client identity remains public
`c447c0761835339a39efdf2ddd8101ebf80db15a`.

Login is the independently published provider
`https://github.com/shuyra-arche/session-login.git` at
`ce8e9007817d7494e05c057e23a7ba907a5d5ba8`, whose own source baseline is
original Source `94e114614bec53424ac55e640fc80cf19a3e7107`. Its
`units/login/PROVENANCE.md`, path mapping, source-to-filtered and
source-to-published commit maps, original authors and file hashes remain the
source-map authority for that Unit; this aggregate does not restate or alter
them. This slice registers that provider in the Cluster: it adds the
`units/login` submodule pin, the `login-declaration` Cargo dependency at the
same full SHA, the ordered `UNITS`/`IMAGE_BUILDS` entries with the
`units/login/server/Cargo.toml` prefix, and host registration tests. No Login
provider code was changed or republished here.

## Additive slice bases

This change stacks on the URL-only Session PR #1 head
`485b30c36be7554459f7afe43d5a70bf14212274`, and records original Session base
`f7c7542acf0bf010c1aac07d36d30a19e0591731` as reported for this stack. Both
branches stay unmodified; this slice is additive on top of the `485b30c3`
tree. The login submodule gitlink, the `.gitmodules` stanza and Cargo lock
regeneration are Root-owned.

## Cluster-owned changes and limits

The Cluster owns only its pure explicit member/build interface, exact Unit
version selection, shared acceptance contract and generated lock. Unit workspaces
are excluded from the Cluster workspace. Original Source/vanllia still owns the
combined composition, selected Deployment, Shell adapters and cross-Cluster
acceptance. Login is registered from its independently published provider;
registration is neither composition nor startup, and real startup, the Account
create/inspect/cancel handover and terminal foreground lending remain pending
Source/Vanllia composition and acceptance work.

Public System API `9e5d63ca536a176583cf1b0054b4ecf7ddd88257`, client/support
`5a792ff2d39901a970925b1a6d63206adf6c16d8`, System runtime
`9001af65f97501dfa490c9397c61d593c871136d` and Kernel
`3aa0a155f825fbf377fc8d04009472ca67cbbf8c` remain the accepted external
identities. No global Unit registry, runtime Cluster type, Source backlink,
provider overlay or giant workspace is introduced.

Cluster/public recursive clean-checkout and new Source SMP4/8 integration have
separate gates. Prior Source56 host cases/12 ELF artifacts under the old
monorepo boundary remain historical and are not carried to changed repository
pins. The previously recorded Account/Terminal identities, host and ELF results
remain historical evidence for those pins, not current runtime proof for this
tree. All 12 runtime gates remain PENDING. No QEMU result or complete
Session/Driver/Vanllia migration is claimed at this publication stage. The
source root had no project LICENSE; no license is invented. Original Unit file
headers and authorship remain in their own public histories and provenance
files.
