# ardosia-raknet

[![Rust](https://img.shields.io/badge/Rust-1.98%2B-000000?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Pre--release-yellow)](#status)

`ardosia-raknet` is an Ardosia-maintained hardfork of `mcbe-rs/raknet-rust`, kept as a standalone asynchronous RakNet transport library for Rust.

It sits below all Minecraft/game-specific layers. MCPE packet definitions, gameplay semantics, world state, and Ardosia application behavior do not belong here.

## Status

The hardfork is pre-release and is actively consumed by `ardosia-network` using an exact Git revision for reproducibility.

Current baseline:

- package name: `raknet-rust`
- upstream version: `0.2.0`
- preserved upstream baseline: `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`
- Ardosia network-pinned revision: `f127fce27a206a51a1d39ffa7a9bbed98d10ea14`
- Rust baseline: `1.98.0`
- license: Apache-2.0

See `UPSTREAM.md` for provenance and fork policy.

## Scope

The hardfork owns:

- UDP socket binding and tuning;
- RakNet offline and connected handshakes;
- sessions and session state;
- reliability, ordering, and sequencing;
- ACK/NACK and retransmission behavior;
- congestion and pacing;
- fragmentation and reassembly;
- sharded transport runtime behavior;
- transport-level rate limiting, abuse controls, and processing budgets;
- low-level transport telemetry;
- RakNet protocol-version compatibility.

It does **not** own:

- MCPE protocol-84 packet definitions/codecs;
- game/session policy;
- world/chunk serialization;
- gameplay state;
- Ardosia application/server lifecycle.

Those responsibilities live above this layer in `ardosia-network`, `ardosia-protocol`, and `ardosia-server`.

## Using the hardfork

Until a deliberate release/publishing policy exists, consumers should pin an exact Git revision rather than a moving branch:

```toml
[dependencies]
raknet-rust = {
    git = "https://github.com/ardosia/ardosia-raknet",
    rev = "<exact-commit-sha>"
}
```

`ardosia-network` follows this policy so transport behavior remains reproducible.

## API surface

The inherited application API lives under `client`, `server`, `listener`, `connection`, and root re-exports. Advanced low-level APIs are namespaced under `raknet_rust::low_level::{protocol, session, transport}`.

No Ardosia-specific game-protocol concepts should be added for convenience. Generic facade concerns belong in `ardosia-network`; MCPE concerns belong above it.

## Ardosia compatibility use

The hardfork remains protocol-version configurable. The current Ardosia server stack uses RakNet protocol `8` and disables the newer handshake-cookie path through generic transport configuration supplied by `ardosia-network`.

That compatibility profile has been exercised by a real MCPE 0.15.10 client through the network/server stack. This does not turn the hardfork into an MCPE-specific library.

## Verification

Run the standalone Rust `1.98.0` gate:

```bash
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --all-targets --all-features -- -D warnings
cargo +1.98.0 test --all-targets
git diff --check
```

Transport behavior changes should be backed by protocol correctness evidence, regression tests, benchmark evidence, or profiling evidence.

Production abuse-control defaults must not be weakened solely to make localhost load-generation artifacts disappear.

## Design policy

Keep this repository reusable and transport-focused:

- changes should describe generic RakNet behavior rather than Ardosia game concepts;
- public low-level configuration should remain explicit rather than hidden behind application assumptions;
- correctness and compatibility take priority over benchmark-only tuning;
- higher layers should consume stable/opaque transport surfaces rather than implementation internals.

## License and provenance

The project remains Apache-2.0 licensed. Upstream history and attribution are preserved; see `UPSTREAM.md`.

The original upstream project does not endorse or support Ardosia.
