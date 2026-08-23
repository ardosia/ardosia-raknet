# ardosia-raknet

[![Rust](https://img.shields.io/badge/Rust-1.98%2B-000000?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Pre--release-yellow)](#status)

`ardosia-raknet` is an Ardosia-maintained hardfork of `mcbe-rs/raknet-rust`, kept as a standalone asynchronous RakNet transport library for Rust.

The repository intentionally stays below the Ardosia game/protocol layers. It owns RakNet and UDP transport mechanics; MCPE packet definitions, gameplay semantics, and server state do not belong here.

## Status

This hardfork is pre-release. The behavior-preserving extraction is complete: `ardosia-network` now consumes the standalone hardfork at the exact verified revision `f127fce27a206a51a1d39ffa7a9bbed98d10ea14`. Future API or transport changes should be reviewed as standalone hardfork work rather than bundled as extraction cleanup.

Current baseline:

- package name: `raknet-rust`
- upstream version: `0.2.0`
- fork baseline: `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`
- network integration revision: `f127fce27a206a51a1d39ffa7a9bbed98d10ea14`
- Rust: `1.98+`
- license: Apache-2.0

See [`UPSTREAM.md`](UPSTREAM.md) for provenance and fork policy.

## Scope

The hardfork owns:

- UDP socket binding and tuning;
- RakNet offline and connected handshakes;
- sessions and session state;
- reliability, ordering and sequencing;
- ACK/NACK and retransmission behavior;
- congestion and pacing;
- fragmentation and reassembly;
- sharded transport runtime behavior;
- transport-level rate limiting, abuse controls and processing budgets;
- low-level transport telemetry;
- RakNet protocol-version compatibility.

It does **not** own MCPE protocol 84 packet definitions, gameplay state, world logic, or Ardosia application/server behavior.

## Using the hardfork

Until a deliberate release/publishing policy exists, consumers should pin an exact Git revision rather than track a moving branch:

```toml
[dependencies]
raknet-rust = {
    git = "https://github.com/ardosia/ardosia-raknet",
    rev = "<exact-commit-sha>"
}
```

`ardosia-network` follows this exact-SHA policy so transport behavior remains reproducible.

## API surface

The inherited application API lives under `client`, `server`, `listener`, `connection` and root re-exports. Advanced low-level APIs are namespaced under `raknet_rust::low_level::{protocol, session, transport}`.

No Ardosia-specific game protocol concepts should be added merely for convenience. If a concept belongs in the stable Ardosia networking facade, it belongs in `ardosia-network` instead.

## Verification

The repository declares Rust `1.98` as its minimum supported toolchain. Run the standalone hardfork gate on that exact toolchain:

```bash
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --all-targets --all-features -- -D warnings
cargo +1.98.0 test --all-targets
```

CI and `rust-toolchain.toml` are pinned to Rust `1.98.0` as well so a future moving `stable` Clippy release cannot turn inherited style lints into unrelated failures.

Transport behavior changes should be backed by protocol correctness evidence, regression tests, benchmark evidence, or profiling evidence. Production abuse-control defaults must not be weakened solely to make localhost load-generation artifacts disappear.

## License and provenance

The project remains Apache-2.0 licensed. Upstream history and attribution are preserved; see [`UPSTREAM.md`](UPSTREAM.md).

The original upstream project does not endorse or support Ardosia.
