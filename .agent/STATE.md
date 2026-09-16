# Current State

Last updated: 2026-09-16
Current milestone: centralized-documentation migration complete
Current branch: `main`
Current network-consumed revision: `55b57787b6715ef2a931631ef4b690e3df0651e5`

## Working
- Standalone generic RakNet transport hardfork.
- Configurable protocol-version compatibility used by Ardosia for RakNet 8.
- Wider Ardosia stack has exercised the pinned transport path with a real 0.15.10 client.
- `.agent/` continuity harness remains local and authoritative for execution state.
- Durable README, upstream provenance, consumer-pin policy, contribution/security guidance, and the branch-only Ardosia integration note are archived in `ardosia/ardosia-docs` with source provenance.
- Re-fetched central copies have blob SHAs identical to the migrated source documents.
- Public README/UPSTREAM/CONSUMER-PINS/CONTRIBUTING/SECURITY files remain in this repository intentionally because they are live consumer, provenance, operational, or security surfaces.
- The unique `docs-runtime-status/ARDOSIA-INTEGRATION.md` duplicate was removed after central verification.

## Partially working / branch state
- `main` is ahead of the exact revision currently consumed by `ardosia-network`; later commits are not automatically consumer-verified.
- `docs-runtime-status` no longer carries unique durable integration prose; it is now only a historical maintenance branch and should not be merged merely to restore centralized documentation.

## Broken / failing
- No transport defect is established by this documentation cleanup.

## Test status
- Central documentation copy verification: **PASS** — re-fetched migrated documents matched original source blob SHAs.
- Branch-only integration-note source cleanup: **PASS**.
- Full standalone RakNet gate: **NOT RUN** in this documentation-only migration round.
- Short hosted soak/CI: **NOT RUN** / not queried in this round.
- Wider server real-client connection smoke through the current pinned stack: **PASS**, but this is not a standalone transport gate.

## Current blocker
None.

## Active work
- Organization-wide documentation migration continues in sibling repositories.
- No RakNet consumer pin movement is implied by this cleanup.

## Important temporary facts
- Do not move the network consumer pin as part of documentation cleanup.
- Do not weaken transport abuse-control defaults to satisfy synthetic localhost load behavior without measured evidence.
- Keep public provenance/security/contribution surfaces local even though canonical archival copies also exist in `ardosia-docs`.
