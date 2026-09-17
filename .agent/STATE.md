# Current State

Last updated: 2026-09-17
Current milestone: stable generic RakNet lower layer; no current identity/map/login runtime delta
Default branch: `main`

## Role
`ardosia-raknet` is the public generic RakNet hardfork used by `ardosia-network`. It owns RakNet mechanics, not Minecraft protocol/gameplay/account semantics.

## Working
- Generic RakNet transport hardfork remains available to the network facade.
- `ardosia-network` currently consumes exact revision `55b57787b6715ef2a931631ef4b690e3df0651e5`.
- Canonical durable Ardosia documentation is centralized in `ardosia-docs`; documentation migration is no longer active work here.
- `.agent/{CONTEXT,STATE,DECISIONS,NEXT}.md` remains the local continuity harness.

## Current convergence impact
The completed identity/map/login findings are above this layer and require no RakNet code change. Do not pull game protocol, identity, inventory, or world semantics into RakNet.

If future evidence identifies a genuine RakNet-8 transport delta, implement and validate it here or persist the exact deferment. Evidence completion alone is not a RakNet runtime PASS.

## Pin/branch state
`main` may contain work newer than the exact network-consumed revision. The consumer pin remains authoritative for claims about the active stack. Do not move it merely because `main` is newer.

## Validation status
- This state-only migration changes no Rust/runtime behavior: standalone Rust validation **NOT RUN**.
- Historical tests/soak results remain historical evidence only.
- Network consumer pin movement: **NOT RUN**.

## Active work
No runtime work is required from the current evidence convergence pass. Revisit only for a concrete RakNet defect, transport requirement, or deliberate validated consumer-pin reconciliation.
