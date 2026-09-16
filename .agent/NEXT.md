# Next Work

1. Finish central documentation migration after `ardosia/ardosia-docs` exists.
   - Migrate README, UPSTREAM, CONSUMER-PINS, contribution/security prose, and the unique `docs-runtime-status` integration note with provenance.
   - Verify central copies before deleting duplicated local docs.

2. Reconcile the stale `docs-runtime-status` branch after central migration.
   - Preserve the useful integration note centrally.
   - Do not merge it merely to restore old documentation files that are being centralized.

3. Leave the exact consumer pin unchanged until a dedicated transport update is requested and validated.

4. For any future RakNet behavior change, run the full Rust 1.98 gate and relevant soak/protocol regression checks; record actual outcomes in `STATE.md`.
