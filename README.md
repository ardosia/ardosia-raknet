# ardosia-raknet

[![Continuous integration](https://github.com/ardosia/ardosia-raknet/actions/workflows/ci.yml/badge.svg)](https://github.com/ardosia/ardosia-raknet/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.98%2B-000000?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Pre--release-yellow)](#status)

`ardosia-raknet` is an Ardosia-maintained hardfork of [`mcbe-rs/raknet-rust`](https://github.com/mcbe-rs/raknet-rust), kept as a standalone asynchronous RakNet transport library for Rust.

The repository intentionally stays below all Minecraft/game-specific layers. It owns RakNet and UDP transport behavior; MCPE packet definitions, gameplay semantics, world state, and Ardosia application behavior do not belong here.

## Status

The hardfork is **pre-release** and is actively consumed by Ardosia through an exact Git revision for reproducibility.

Current baseline:

- package/crate name: `raknet-rust`
- upstream version lineage: `0.2.0`
- preserved upstream baseline: `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`
- current Ardosia network pin: `f127fce27a206a51a1d39ffa7a9bbed98d10ea14`
- Rust baseline: `1.98.0`
- license: Apache-2.0

See [`UPSTREAM.md`](UPSTREAM.md) for the exact fork provenance and maintenance policy.

> [!IMPORTANT]
> The upstream project owns the `raknet-rust` package published on crates.io. This Ardosia hardfork is **Git-only for now** and is not a separate crates.io release of that package. If you want this fork, depend on this Git repository explicitly and pin an exact revision.

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

- Minecraft or MCPE packet definitions/codecs;
- game/session admission policy;
- world/chunk serialization;
- gameplay state;
- Ardosia application/server lifecycle.

Those responsibilities remain above the transport boundary.

## Using the hardfork

Pin an exact commit rather than a moving branch:

```toml
[dependencies]
raknet-rust = {
    git = "https://github.com/ardosia/ardosia-raknet",
    rev = "<exact-commit-sha>"
}
```

For example, the Ardosia networking facade currently pins the transport revision shown in the status section above. Consumers should choose and record the revision they have actually verified rather than copying a moving `main` reference.

### Minimal server

```rust,no_run
use raknet_rust::server::{RaknetServer, RaknetServerEvent};

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut server = RaknetServer::bind("0.0.0.0:19132".parse().unwrap()).await?;

    while let Some(event) = server.next_event().await {
        if let RaknetServerEvent::Packet { peer_id, payload, .. } = event {
            server.send(peer_id, payload).await?;
        }
    }

    Ok(())
}
```

### Minimal client

```rust,no_run
use raknet_rust::client::{RaknetClient, RaknetClientEvent};

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut client = RaknetClient::connect("127.0.0.1:19132".parse().unwrap()).await?;

    while let Some(event) = client.next_event().await {
        match event {
            RaknetClientEvent::Connected { .. } => client.send(b"hello").await?,
            RaknetClientEvent::Packet { .. } | RaknetClientEvent::Disconnected { .. } => break,
            _ => {}
        }
    }

    Ok(())
}
```

## API surface

The inherited application API lives under `client`, `server`, `listener`, `connection`, and root re-exports. Advanced low-level APIs are namespaced under `raknet_rust::low_level::{protocol, session, transport}`.

No Ardosia-specific game-protocol concepts should be added for convenience. Generic RakNet behavior belongs here; application-specific policy belongs in the consuming application or facade.

Because this fork is pre-release, consumers should review the exact revision they pin rather than assuming API stability from the upstream `0.2.0` package version alone.

## Ardosia compatibility use

The hardfork remains protocol-version configurable. Ardosia currently uses RakNet protocol `8` and disables the newer handshake-cookie path through generic transport configuration supplied by its private networking facade.

That compatibility profile has been exercised by a real Minecraft: Pocket Edition 0.15.10 client through the wider Ardosia stack. This evidence does not make the hardfork Minecraft-specific and is not a universal capacity or compatibility claim.

## Verification

The repository pins Rust `1.98.0`. Before submitting a transport change, run:

```bash
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --all-targets --all-features -- -D warnings
cargo +1.98.0 test --all-targets
git diff --check
```

The hosted CI additionally runs a short RakNet soak on Ubuntu and checks formatting/Clippy on both Ubuntu and Windows.

Transport behavior changes should be backed by protocol correctness evidence, regression tests, meaningful benchmark evidence, or profiling evidence. Production abuse-control defaults must not be weakened solely to make localhost load-generation artifacts disappear.

## Contributing and security

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) before proposing changes. Keep changes generic to RakNet and separate unrelated API, performance, and behavior changes so they can be reviewed independently.

Do **not** report suspected security vulnerabilities in public issues. Follow [`SECURITY.md`](SECURITY.md) and use GitHub private vulnerability reporting when available.

## Support boundary

This repository is maintained by the Ardosia project as a hardfork. Bugs in behavior changed by this fork should be reported here; issues that reproduce unchanged against the preserved upstream baseline may also be relevant to the upstream project.

Please do not ask the original upstream maintainers to support Ardosia-specific changes. The original project does not endorse or support this fork.

## License and provenance

This repository remains licensed under the Apache License 2.0. See [`LICENSE`](LICENSE).

Upstream history and attribution are preserved. See [`UPSTREAM.md`](UPSTREAM.md) for the baseline revision and fork relationship.