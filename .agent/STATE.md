# Current State

Last updated: 2026-09-16
Current milestone: workflow/documentation migration
Current branch: `main`
Current head before harness migration: `c1d42a1d2527f85152779db59c7d44ae929c13a2`
Current network-consumed revision: `55b57787b6715ef2a931631ef4b690e3df0651e5`

## Working
- Standalone generic RakNet transport hardfork.
- Configurable protocol-version compatibility used by Ardosia for RakNet 8.
- Fork includes Ardosia-maintained transport correctness/runtime hardening while preserving generic transport boundaries.
- Wider Ardosia stack has exercised the pinned transport path with a real 0.15.10 client.

## Partially working / branch state
- `main` is ahead of the exact revision currently consumed by `ardosia-network`; those later commits are not automatically consumer-verified.
- `docs-runtime-status` has one unique `ARDOSIA-INTEGRATION.md` documentation commit but is behind newer main docs and should be reconciled into centralized docs, not blindly merged as runtime code.

## Broken / failing
- No transport defect is established by the current cleanup evidence.

## Test status
- Full standalone RakNet gate: **NOT RUN** in this workflow migration round.
- Short hosted soak/CI: **NOT RUN** / not queried in this round.
- Wider server real-client connection smoke through the current pinned stack: **PASS**, but this is not a standalone transport gate.

## Current blocker
- Central docs migration is **BLOCKED** until `ardosia/ardosia-docs` exists; the available GitHub connector cannot create repositories.

## Active work
- Install `.agent/` harness.
- Centralize README/UPSTREAM/CONSUMER-PINS/contribution/security/integration documentation.

## Important temporary facts
- Do not move the network consumer pin as part of documentation cleanup.
- Do not weaken transport abuse-control defaults to satisfy synthetic localhost load behavior without measured evidence.
