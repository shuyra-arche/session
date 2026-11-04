# Terminal `f07` → `d120` intake validation

The base is Session PR #2 head `3534b3a868f49424fd34213960a8201f8a914c1a`,
branch `codex/session-login-registration`. The new work branch
`codex/session-terminal-uart-intake` is based exactly on that commit. Both
branches remain unmodified by this slice except for the additive work branch
itself.

## Pins

| Input | Revision |
| --- | --- |
| Base | Session PR #2 head `3534b3a868f49424fd34213960a8201f8a914c1a` (`codex/session-login-registration`) |
| Work branch | `codex/session-terminal-uart-intake` |
| Terminal submodule clone | `https://github.com/shuyra-arche/session-terminal.git` at `d120e3ddf4b4a6c00685f32310897622d3440dd5` |
| Terminal declaration dependency | `https://github.com/shuyra-arche/terminal.git` at `d120e3ddf4b4a6c00685f32310897622d3440dd5` |
| Baseline comparator | Session base `3534b3a8…` with Terminal `https://github.com/shuyra-arche/terminal.git` at `f07a0954f65472e9a16e878e52c5f1d34146bc64` |
| Account | `account.git` `7dc701facdbe6368fbc09f7429920ad49dacdfca` |
| Login | `session-login.git` `ce8e9007817d7494e05c057e23a7ba907a5d5ba8` |
| Unit | `unit.git` `90396f0fda376e0693fb1b9553aa21864d241057` |
| Terminal API | `terminal.git` `c447c0761835339a39efdf2ddd8101ebf80db15a` |
| Nested UART source | `driver-uart.git` `298bc8809f297f46bc7592aebc96540b7b492efc` (was `uart.git` `b690431496b0beaeb5fb7cafb42e60b4c8b38c9d`) |
| System API | `system.git` `9e5d63ca536a176583cf1b0054b4ecf7ddd88257` |

The reproducible comparison is fixed base `3534b3a8…` with Terminal at `f07`
against this work branch with Terminal at `d120`. The Terminal clone remote
`session-terminal.git` and the Cargo dependency URL `terminal.git` are
distinct sources and are both retained.

## Invariants

- The implementation delta is exactly one line in `cluster/Cargo.toml`: the
  `terminal-declaration` revision. Its URL remains the canonical
  `https://github.com/shuyra-arche/terminal.git`.
- `account-declaration` `7dc701fa…`, `login-declaration` `ce8e9007…` and
  `unit` `90396f0f…` source lines remain byte-identical.
- All cluster `.rs` and test bytes, `.gitmodules`, `rust-toolchain.toml`, the
  workspace declaration and `ACCEPTANCE.md` are frozen.
- Every Unit worktree is the exact byte tree of its own selected pinned
  provider: Account `7dc701fa…`, Terminal `d120e3dd…`, Login `ce8e9007…`. Units
  are not all equal to their former revisions; Terminal's provider
  `server/Cargo.toml` and nested lock carry the intentional selected UART
  delta described below.
- Terminal `declaration/src/lib.rs` and `declaration/Cargo.toml` are byte-equal
  between `f07` and `d120`; `UNIT`, `BUILD`, `E_TERMINAL` and the retained
  `cluster/tests/registration.rs` assertions are unchanged.
- Terminal moves exactly its two production UART edges
  `uart.git@b6904314…` → `driver-uart.git@298bc880…`. Equal UART source bytes
  support retained source-level behavior; no object/item byte identity is
  claimed. The canonical `terminal.git` Cargo source and the distinct
  `session-terminal.git` clone remote are both retained.
- No wrapper type, new test, second checker or mirror is introduced; Terminal's
  existing `tests/uart-identity/check.py` is the consumer-graph authority.
- All 12 runtime gates in `ACCEPTANCE.md` remain PENDING. No QEMU, product
  integration, Login startup or foreground-lending result is claimed.

## Verified local candidate (Root-run)

Root executed these checks on the local candidate checkout with explicitly
selected installed `nightly-2026-10-09`. The preserved declared
`nightly-2026-09-04` was not exercised, so these results are not
declared-channel proof.

| Check | Result |
| --- | --- |
| Locked/offline metadata, aggregate and all three Units | All passed |
| Cluster locked/offline metadata resolves one `terminal-declaration` at `d120e3dd…` | Passed |
| Session `Cargo.lock` diff is exactly the one `terminal-declaration` source line | Passed |
| Terminal nested lock diff is only the two UART source lines | Passed |
| Host cases: aggregate 3, Account API 4 / configuration 4 | All passed |
| Host cases: Terminal API 5 / canonical 6 / connection 5 | All passed |
| Host case: Login 3 | Passed |
| Aggregate RISC-V debug / release (libraries) | Both built |
| Account / Terminal / Login RISC-V debug / release executables | All six built |
| Freeze proof over `cluster/src`, `cluster/tests`, `.gitmodules`, `rust-toolchain.toml`, workspace manifest and `ACCEPTANCE.md` | Passed |
| Account and Login gitlinks unchanged; `units/terminal` gitlink at `d120e3dd…` | Passed |
| Each Unit worktree equals the exact bytes of its own selected provider | Passed |
| Nested `tests/uart-identity/check.py`, exit 0, single UART family at `driver-uart.git@298bc880…` | Passed |
| Consumer graph: 5 locked/host/RV/identity checks | All passed |
| Old mixed Terminal + `uart.git@b690` consumer graph rejection | Actual exit 1 as expected |

The 23 expected-success native checks and the 5 expected-success consumer checks
all returned their expected exit status. The baseline rejection consumed a real
pre-reconciliation Terminal plus selected UART/Driver consumer graph, not a
synthetic input, and returned the intended exit 1.

The aggregate graph is 16 declaration-only packages with no server/client
runtime dependency and one canonical `unit`. The individual actual normal
closures are: Terminal 28 packages with canonical API `c447c076…` plus the
`driver-uart.git@298bc880…` UART pair; Account 26 packages with canonical
Terminal API `c447c076…` and its own in-tree `account-api`, which is not equated
with the Git `account-api` source identity; Login 40 packages with canonical
Terminal client/API `c447c076…` and Account client/API `7dc701fa…`, zero `unit`
crates in its normal closure, and its bin-only root checked as the actual
workspace. A combined consumer root of 45 packages rooted on Session (this local
candidate cluster), Terminal (`terminal.git@d120e3dd`), UART
(`driver-uart.git@298bc880`) and Driver (`driver.git@bb3427e1`) resolves the
correct declarations, one canonical `unit`, and exactly one identity each for
all four UART family members at `driver-uart.git@298bc880…`. Existing provider
warnings remain; no source or test was changed to silence them.

The aggregate RISC-V debug/release artifacts are libraries, not executable
images. Each actual Account/Terminal/Login executable was built through its own
Unit workspace link configuration. All six are static RISC-V `ET_EXEC` with
image base `0x10000`, entry equal to `_start`, and exactly one startup,
clean-return, TLS bootstrap, panic handler and allocator entry. The Terminal
release artifact is 341720 bytes, SHA-256
`bbf335ed4871fd02fd1441f2098b672ace11a45bc29cbd2d2455d8e897b28955`; hashes
identify these artifacts and are not a universal reproducible-build claim.

## Pending after publication

The checks above are the currently verified local candidate and recursive
checkout. They are not proof of the future public repository. After this slice
is published, Root must independently run and report the Git-only public
checkout: a clean checkout of the resulting public Session head with Git-only
Session roots, no local path dependency or override, locked/offline metadata,
the host cases, both RISC-V profiles and the combined consumer graph resolving
the published Session commit. Those public-checkout results are not yet run and
are not claimed here.

All 12 runtime gates in `ACCEPTANCE.md` remain PENDING. No QEMU, initrd, real
startup, Account handover or terminal foreground lending was run, and no
product integration is claimed. Future Session/Vanllia work must adopt the
resulting public Session head.

## Review group

- Namer: `session-c7dc5d75-a2e6-4a6c-9f0f-7248c0372621`.
- Sole worker: `session-8209055d-b1de-4b88-bd79-b66203da1c95`.
- Independent reviewer: `session-65850f5d-029e-4b1a-934e-ea76c26f0cc5`,
  PASS with no blocking findings. Review used actual files and Root's native
  evidence; the reviewer did not execute native commands.
