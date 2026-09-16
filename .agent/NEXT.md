# Next Work

1. Leave the exact consumer pin unchanged until a dedicated transport update is requested and validated.
   - Newer `main` commits are not automatically part of the Network/server stack.
   - Any pin move must identify old/new SHAs and the transport behavior/evidence between them.

2. Treat `docs-runtime-status` as historical branch state.
   - Its unique integration prose is now preserved in `ardosia-docs` and the source duplicate has been removed.
   - Do not merge the branch merely to restore centralized documentation.

3. Continue normal RakNet maintenance through evidence-backed issues and focused PRs.
   - For behavior changes, run the full Rust 1.98 gate plus relevant soak/protocol regression checks.
   - Record actual PASS/FAIL/BLOCKED/NOT RUN outcomes in `STATE.md`.

Completed prerequisite: RakNet durable documentation migration is complete and centrally verified while public consumer/provenance/security surfaces remain intentionally local.
