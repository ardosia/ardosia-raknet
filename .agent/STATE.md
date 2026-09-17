# Current State

Last updated: 2026-09-17
Current milestone: repository hygiene; RakNet runtime work paused
Default branch: `main`

## Role
`ardosia-raknet` is the public generic RakNet hardfork used by `ardosia-network`. It owns RakNet mechanics, not Minecraft protocol, gameplay, account identity, inventory, or world semantics.

## Working
- Generic RakNet transport hardfork remains available to the network facade.
- `ardosia-network` currently consumes exact revision `55b57787b6715ef2a931631ef4b690e3df0651e5`.
- Canonical durable Ardosia documentation is centralized in `ardosia-docs`.
- `.agent/{CONTEXT,STATE,DECISIONS,NEXT}.md` remains the local continuity harness.

## Branch/pin state
The exact network-consumed revision remains authoritative for active-stack claims. `docs-runtime-status` is historical maintenance state, not active work. The temporary workflow reconciliation branch is merged history only.

## Validation
- Workflow/state reconciliation to `main`: **PASS**.
- Standalone Rust validation: **NOT RUN** in this hygiene pass.
- Network consumer pin movement: **NOT RUN**.
- Runtime code changes: **NOT RUN**.

## Active work
Repository hygiene and agent-state reconciliation only. Revisit runtime behavior only after explicit user direction or for a concrete RakNet transport defect.
