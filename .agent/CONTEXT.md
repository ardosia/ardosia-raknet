# Repository Context

## Purpose
`ardosia-raknet` is the standalone Ardosia-maintained hardfork of `mcbe-rs/raknet-rust`, kept below all Minecraft/game-specific layers.

## Ownership boundary
This repository owns generic RakNet/UDP transport behavior: sockets, offline/connected handshakes, sessions, reliability/ordering/sequencing, ACK/NACK, retransmission, pacing/congestion, fragmentation, sharded runtime behavior, transport rate limits/budgets, telemetry, and configurable RakNet protocol compatibility.

It does not own Minecraft packet definitions, gameplay/session policy, world state, or Ardosia application lifecycle.

## Baseline and consumer pin
- preserved upstream baseline: `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`
- current `ardosia-network` consumer pin: `55b57787b6715ef2a931631ef4b690e3df0651e5`
- repository `main` may be ahead of the consumer pin; do not equate moving head with a verified consumer revision.
- Rust baseline: 1.98.0

## Validation
```bash
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --all-targets --all-features -- -D warnings
cargo +1.98.0 test --all-targets
git diff --check
```
Hosted CI also includes a short soak and cross-platform formatting/Clippy coverage.

## Documentation location
Durable fork/provenance/integration documentation is being centralized in `ardosia/ardosia-docs`. Keep `LICENSE`, code/build/CI artifacts, and `.agent/` locally. Do not delete documentation before central copies are persisted and verified.
