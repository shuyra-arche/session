# Session joint acceptance

Status: **PENDING** under the two-layer Unit/Cluster layout. No prior monorepo
result is carried forward, and no structural mirror test substitutes for the
actual gates below.

## External Source-owned requirements

- Login is registered in this Cluster from its independently published provider
  `session-login` at `ce8e9007817d7494e05c057e23a7ba907a5d5ba8`. That
  registration selects a declaration and image build; composition, real startup
  and real terminal/Account acceptance remain pending.
- Shell, Driver and System runtime/declaration inputs remain externally owned at
  their accepted public identities.
- Actual combined Session composition, selected Deployment and AccountRig
  remain Source/Vanllia-owned.

The Cluster host tests in `cluster/tests/registration.rs` cover only ordered
registration, provider pointer identity and prefixed image coordinates. Real
Login startup, the Account create/inspect/cancel handover, terminal foreground
lending and the fault probes remain unexercised here and are not replaced by
those tests.

## Root native/runtime gates

Root independently selects and runs the real commands after the Account,
Terminal and Login Unit and Session aggregate public SHAs, generated Cluster
lock, and recursive clean checkout are proven. Still pending:

- Product terminal-session and shell-smoke, each at SMP4 and SMP8 (4 runs).
- `accept`, `system-fault`, `memory-transfer` and driver `iommu-ruin`, each at
  SMP4 and SMP8 (8 runs).

The complete targeted matrix is12 runs across six suites, all still PENDING.

Each gate retains its original raw exit and strict judge under the new layout.
No fake pass, copied runtime, path override, or new mirror structural test is
accepted in place of these gates.
