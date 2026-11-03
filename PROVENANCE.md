# Provenance

Source: https://github.com/anreonyr/sqware.git
Frozen source revision: `293c15fc97a9d9a4295810d10fe804e4cdec63c4`

This Session Cluster introduction owns the Account Unit and the pure
`session-cluster` member slices. Terminal and Login remain pending/Source-owned;
the actual combined composition and selected Deployment remain Source/Vanllia.

The 20 Account production/test files were mechanically copied byte-identically
from the retained checkpoint `/workspace/providers/account` into
`units/account`. The exact path/hash map is
`evidence/account/cluster-relocation-map.json` in the Root workspace. The old
checkpoint remains read-only and is not published here.

Account preserves the original Account API, client, server bodies and
declaration facts. Repository-boundary changes only:

- added the cluster root workspace and `cluster` package;
- changed the Account `BUILD` manifest to `units/account/server/Cargo.toml`;
- corrected the API raw-layout test's expected length from 21 to the actual
  20 encoded bytes (`1 + 8 + 1 + 2 + 8`).

Original authors recorded by the source history:

- Serial Device <anreonyr@gmail.com>
- anreonyr <88045926+anreonyr@users.noreply.github.com>
- anreonyr <anreonyr@gmail.com>

The source root has no project LICENSE; this snapshot adds no invented license
and does not rewrite Source history.

## Frozen original hashes and attribution

All entries below are Source https://github.com/anreonyr/sqware.git at `293c15fc97a9d9a4295810d10fe804e4cdec63c4`, before package-boundary adaptation. Root retains the original bytes; public readers can reproduce them from that commit.

| Original source | Initial copied ownership | SHA-256 |
| --- | --- | --- |
| `programs/src/service/account/api/Cargo.toml` | `units/account/api/Cargo.toml` | `10060260e4760d20b505e513d8ac8a29517934b92169ba0c10b76f466ac6df5c` |
| `programs/src/service/account/api/src/frame.rs` | `units/account/api/src/frame.rs` | `ef8e1353e5bb57a086be332ef347cb265ff7bbd6aefb7f020f459246527156e5` |
| `programs/src/service/account/api/src/lib.rs` | `units/account/api/src/lib.rs` | `3fe987c185a1bbc9e197ef0cde10bcf6062a23efc3add7328b1d7685f56b96b9` |
| `programs/src/service/account/client/Cargo.toml` | `units/account/client/Cargo.toml` | `621b089d6fb797c38f8aa4a24ea4c5820bb99473bc094a9be29b561e5f728980` |
| `programs/src/service/account/client/src/lib.rs` | `units/account/client/src/lib.rs` | `7690b6af51dc45d302d874d6b388ccfe8c4b4a3d29b0de0ae8b91361a56755ee` |
| `programs/src/service/account/mod.rs` | `units/account/server/src/lib.rs` | `52b3ca5d1bbf90bfe238e8ebcbd5c9dc05b985202885f0f8d95b1beab2a75ec0` |
| `programs/src/service/account/main.rs` | `units/account/server/src/main.rs` | `68060f71617db977ee2169678ea1a210ff3f51302482b6dbd9f155f57b7b540d` |
| `crates/composition/src/units/account.rs` | `units/account/declaration/src/lib.rs` | `a00f24174d92dd3b2bb9ebd7fa7c27a03a3170525c88dc4a6f2ece8d35ecd960` |

Original source-history authors:

- Serial Device <anreonyr@gmail.com>
- anreonyr <88045926+anreonyr@users.noreply.github.com>
- anreonyr <anreonyr@gmail.com>

The original Source root has no project LICENSE. This snapshot invents none and does not rewrite Source or previously published provider history. The pre-Cluster Account staging tree remains an unpublished checkpoint; Session is the current repository boundary approved by the user.
