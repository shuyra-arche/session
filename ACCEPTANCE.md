# Session joint acceptance

Status: **PENDING** under the two-layer Unit/Cluster layout. No prior monorepo
result is carried forward, and no structural mirror test substitutes for the
actual gates below.

## External Source-owned requirements

- Login remains Source-owned; no Login Unit has been migrated.
- Shell, Driver and System runtime/declaration inputs remain externally owned at
  their accepted public identities.
- Actual combined Session composition, selected Deployment and AccountRig
  remain Source/Vanllia-owned.

## Root native/runtime gates

Root independently selects and runs the real commands after the Account Unit and
Session aggregate public SHAs, generated Cluster lock, and recursive clean
checkout are proven. Still pending:

- Product terminal-session and shell-smoke, each at SMP4 and SMP8 (4 runs).
- `accept`, `system-fault`, `memory-transfer` and driver `iommu-ruin`, each at
  SMP4 and SMP8 (8 runs).

The complete targeted matrix is12 runs across six suites, all still PENDING.

Each gate retains its original raw exit and strict judge under the new layout.
No fake pass, copied runtime, path override, or new mirror structural test is
accepted in place of these gates.
