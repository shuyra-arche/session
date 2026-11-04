# Login registration validation

This slice is based exactly on Session URL-only Draft PR #1 head
`485b30c36be7554459f7afe43d5a70bf14212274`, branch
`session/url-only-submodule-remotes`. The original Session branch remains
`f7c7542acf0bf010c1aac07d36d30a19e0591731`. Neither branch is modified.

Root executed the following checks on 2026-11-04 with explicitly selected
installed `nightly-2026-10-09`. The preserved declared
`nightly-2026-09-04` was not exercised.

| Check | Result |
|---|---|
| Host aggregate registration contracts | 3 passed |
| Account API / configuration host suites | 4 / 4 passed |
| Terminal API / canonical / connection host suites | 5 / 6 / 5 passed |
| Login host suite | 3 passed |
| Cluster locked/offline metadata | Passed |
| Cluster RISC-V debug / release | Both pure libraries built |
| Nested Login RISC-V debug / release | Both actual program ELFs built |
| Existing external lock tuples | All 14 preserved |
| Existing package edges, resolved graph and features | Preserved |
| New cluster dependency | Only pinned `login-declaration` |
| Account and Terminal build blocks and Cargo source lines | Byte-identical |
| All three nested Unit worktrees | Exact pinned bytes, clean |

The aggregate graph has one canonical `unit` identity, and no server/client
runtime package or `sqware` dependency. GitHub clone remotes in `.gitmodules`
use renamed provider repositories; their established Cargo source URLs and
full revisions remain unchanged. Cargo URL equivalence is not assumed.

The three aggregate host tests check the exported declaration references,
order, build coordinates and provider lifecycles. Terminal retains
`Ending::Resident`, `Ready::Signal(READY_MARK)` and its Operator/UART
dependencies. Login retains its provider's `PROGRAM`, `Ending::Told`,
`Ready::OnSpawn` and declared dependencies. Both earlier Unit gitlinks remain
unchanged; only `units/login` is added at
`ce8e9007817d7494e05c057e23a7ba907a5d5ba8`.

The cluster artifacts are RISC-V libraries, not executable images. The nested
Login image's registered manifest resolves to the provider's actual
`server/Cargo.toml`; builds use that independent workspace and its own fixed
link configuration. Both Login ELFs are static RISC-V `ET_EXEC`, image base
`0x10000`, with entry equal to `_start` and exactly one startup, clean-return,
TLS bootstrap, panic handler and allocator entry. The release artifact is
359224 bytes, SHA-256
`a1c122b25bb4eb09ddd54cd8b55942c7b50bc61283eaed4262e3d9441b3cee8e`.

Terminal connection tests retain existing unused-import warnings. No Unit
source or test assertion was changed to remove them.

This PR has a fresh genuine OpenCode Go `deepseek-v4.1-flash` group:

- Namer: `session-2e5a1972-294e-4051-8076-bece71781e30`.
- Sole worker: `session-f8ade923-b87c-4263-b25f-ce5d5a45e4d8`.
- Independent reviewer: `session-3e83ff19-298d-4f86-8e48-76bc5625bc80`,
  PASS with no blocking findings. Review covers code and Root's recorded
  evidence; the reviewer did not execute native commands.

Role tools expose constrained filesystem operations; Unit submodules are
read-only to the roles. Root owns gitlinks, generated locks, native commands
and publication. Local evidence retains actual API response model IDs,
session IDs and tool outcomes separately from Root's checks.

No QEMU, initrd assembly, real startup or Account/terminal handover was run.
Source and Vanllia remain untouched. All 12 joint runtime acceptance gates
in `ACCEPTANCE.md` remain PENDING; these native contracts do not replace them.
