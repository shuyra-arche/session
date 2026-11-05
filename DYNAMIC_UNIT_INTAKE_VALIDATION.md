# Dynamic Unit TOML provider intake

This slice starts at Session
`87abc22d08ffe311b7e43d915240d318642d510c` on the new branch
`codex/dynamic-unit-intake`. It updates only the three provider gitlinks and
adds this validation record. Existing scenarios are not migrated.

| Provider | Public origin | Selected commit | Discovered document |
| --- | --- | --- | --- |
| Account | `https://github.com/shuyra-arche/session-account.git` | `17e797db98f302bd63e135a6c4b4a7850936e12d` | `units/account/unit.toml` |
| Terminal | `https://github.com/shuyra-arche/session-terminal.git` | `d3fd8cf5ea6d84c1a4b7ae5e289631c071a388e5` | `units/terminal/unit.toml` |
| Login | `https://github.com/shuyra-arche/session-login.git` | `235f72dc197cab9252c6a78964bcdc46feb3a264` | `units/login/server/unit.toml` |

Each selected commit was fetched directly from its own public `origin`.
Its tree equals the reviewed local checkpoint below; the differing commit
metadata does not change provider bytes.

| Provider | Preserved local checkpoint | Equal local/public tree |
| --- | --- | --- |
| Account | `cde0d067e55f0fc89659e3fbd65fb3077193880d` | `76bf79f3f9c768f8cbff79c24e29c3b74ca15cec` |
| Terminal | `c9b20194eef7b25dbfbceadc511a44d3462ee5e6` | `697fa5f44f2be7bd589d67383cb4b824cb30dbb6` |
| Login | `e6993f7f75999c9834b6ac993f120b6c923b2f11` | `fd8b0517a193840a8f81f0b554b3baadc1e4f88c` |

Before alignment, each provider worktree was clean. Account and Terminal
checkpoints remain at `refs/backup/session-dynamic-unit-intake-before-public`
in their own repositories. Login's current checkpoint is retained at
`refs/backup/session-dynamic-unit-intake-login-workspace-before-public`;
its original backup also remains. Each `git update-ref` transaction created
the backup and updated `codex/unit-toml-runtime` using its expected old commit.
The three worktrees remained clean at the selected public commits. No reset
or force push was used.

## Initial verification on 2026-11-05

Checks used `/workspace/.vanllia-tools/env.sh`, selecting installed
`nightly-2026-10-09`; the repository's declared toolchain was not changed.
The following initial checks used Login
`12c147035e0d23520fe6ebcd94b5e4029b359f5f`, before the corrective intake below.

| Check | Result |
| --- | --- |
| Public commit/tree identity, checkpoint comparison and clean provider worktrees | Passed for all three providers |
| Recursive `unit.toml` discovery, excluding `.git` and `target`, with Python `tomllib` | Exactly the three documents above; unique IDs `account`, `terminal`, `login`, each `schema = 1` and `creation = "startup"` |
| `cargo metadata --locked --offline --format-version 1` for Session and each provider workspace | All four passed after cache population |
| Aggregate manifest/lock, `cluster/`, `.gitmodules`, declared toolchain, README, provenance and acceptance files against the base | Unchanged |

Initial offline metadata attempts lacked the old aggregate
`session-login.git@ce8e9007...` checkout and Login's `libc 0.2.190` package.
Locked online metadata populated these caches; the subsequent offline runs
passed without changing any manifest or lock. Metadata output is stored only
under the ignored `target/dynamic-unit-intake/` directory.

The existing `cluster/Cargo.toml`, aggregate lock and Rust registration still
select the old declaration graph. Its metadata check confirms that existing
graph remains resolvable; it does not validate the new TOML template graph or
perform dynamic composition. TOML discovery here checks syntax and document
identity only. No centralized static registration was added.

## Login test-workspace correction

The follow-up starts at public Session
`22e2fc197814d4b215531dbdeaa0f62f57a01c8d` and changes only its Login gitlink
and this record. Login `235f72dc...` isolates the new TOML tests in the
independent `tests/unit-document` workspace, excluded from production, and
restores the production lock. This avoids the Unit alias conflict under the
full product's local Unit patch. Production source and Unit TOML are unchanged.

[Login's validation record](units/login/VALIDATION.md) records three new TOML
tests, three existing Login tests and the actual local Unit overlay check,
all reviewed by its original reviewer. This aggregate follow-up verified the
fetched public/checkpoint tree identity, clean worktree and frozen scope;
it did not repeat those tests or metadata for the new Login pin.

System is the core boot Unit, declared by its own `server/unit.toml` with
`id = "system"` and always packaged by the host. Operator and Identity remain
its internal bootstrap threads.
All Unit templates are packaged, including the Shell template with
`creation = "requested"`; Shell instances are created on demand. Account's
Shell requirement selects template closure and does not itself create a Shell
instance. These are integration requirements, not runtime results of this
Session check.

No provider host/RISC-V suites were repeated in this aggregate slice. No
guest, QEMU, SMP, initrd, real startup, handover or foreground-lending run was
performed. The existing acceptance gates remain pending.
