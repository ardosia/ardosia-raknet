# Upstream provenance

`ardosia-raknet` is an Ardosia-maintained hardfork of [`mcbe-rs/raknet-rust`](https://github.com/mcbe-rs/raknet-rust).

## Baseline

- Upstream repository: `mcbe-rs/raknet-rust`
- Upstream crate version: `0.2.0`
- Fork baseline revision: `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`
- Baseline date: 2026-03-01
- Upstream license: Apache-2.0

The `main` branch was seeded from that exact upstream commit with its ancestry preserved. Ardosia-specific transport changes are maintained as ordinary commits after the baseline rather than being squashed into imported upstream history.

## Scope

The hardfork remains a standalone RakNet transport library. Ardosia-specific game protocol and server behavior belong in `ardosia-protocol`, `ardosia-network`, or the application layer rather than this crate.

Changes to RakNet behavior should remain evidence-driven and covered by transport-level tests. Production abuse-control defaults must not be weakened solely to satisfy localhost load-generation artifacts.
