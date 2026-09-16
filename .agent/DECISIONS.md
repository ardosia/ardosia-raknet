# Decisions

## D-0001 — Keep the hardfork generic RakNet transport
Status: Accepted
Date: 2026-09-16

### Decision
Keep Minecraft/game-specific semantics out of `ardosia-raknet`; application and protocol behavior belong in higher layers.

## D-0002 — Consumers pin verified exact revisions
Status: Accepted
Date: 2026-09-16

### Context
Repository `main` can move independently of a consumer's validated transport revision.

### Decision
Ardosia consumers record and pin the exact RakNet commit they have actually verified rather than following moving `main`.

### Consequences
A newer repository head is not automatically production-approved.

## D-0003 — Preserve upstream provenance
Status: Accepted
Date: 2026-09-16

### Decision
Maintain upstream ancestry, attribution, and baseline information while keeping Ardosia hardfork fixes reviewable as generic transport behavior.

## D-0004 — Centralize durable documentation
Status: Accepted
Date: 2026-09-16

### Decision
Move durable fork/consumer/integration/contribution/security prose to `ardosia/ardosia-docs` after verified migration. Keep `.agent/` locally.

## D-0005 — PROJECT_WORKFLOW.md governs substantial work
Status: Accepted
Date: 2026-09-16

### Decision
Use inline single-agent execution, targeted state recovery, exact validation labels, and durable `.agent/` state from the Project workflow.
