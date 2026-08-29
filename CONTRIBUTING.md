# Contributing to ardosia-raknet

Thank you for helping improve the Ardosia RakNet hardfork.

This repository is intentionally a standalone RakNet transport library. Contributions should keep the boundary generic: UDP/RakNet handshakes, reliability, ordering, retransmission, congestion/pacing, fragmentation, runtime behavior, transport telemetry, and abuse controls belong here. Minecraft packets, gameplay policy, world state, and Ardosia application behavior do not.

## Before you start

Read:

- [`README.md`](README.md) for status, scope, usage, and verification;
- [`UPSTREAM.md`](UPSTREAM.md) for the preserved upstream baseline and fork relationship;
- [`SECURITY.md`](SECURITY.md) before reporting a vulnerability.

Search existing issues and pull requests first. For behavior changes, public API changes, protocol compatibility changes, or performance work that may alter runtime policy, open an issue before implementation so the intended scope and evidence can be reviewed.

## Development baseline

The repository pins Rust `1.98.0` in `rust-toolchain.toml`.

Run the complete local gate before requesting review:

```bash
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --all-targets --all-features -- -D warnings
cargo +1.98.0 test --all-targets
git diff --check
```

For transport/runtime changes, also run the relevant focused tests and, when appropriate, the short soak used by CI:

```bash
cargo run --quiet --example raknet_soak -- --sessions=64 --ticks=120 --payload-bytes=120
```

Hosted CI checks formatting and Clippy on Ubuntu and Windows, runs the test suite on Ubuntu, and runs the short soak.

## Change discipline

Keep pull requests focused. Separate unrelated API, correctness, performance, cleanup, and dependency work whenever they can be reviewed independently.

Transport behavior changes should include concrete evidence appropriate to the claim:

- regression tests for correctness or protocol behavior;
- malformed-input tests for parser/decoder changes;
- focused integration coverage for handshake/session/reliability behavior;
- benchmark or profiling evidence for performance claims;
- explicit before/after reasoning for abuse-control, rate-limit, queue, or processing-budget changes.

Do not weaken production abuse controls merely to improve localhost benchmark results. Historical Ardosia testing has demonstrated that process limits and shared-source-IP artifacts can masquerade as transport capacity limits.

## Upstream and fork responsibility

The preserved upstream baseline is documented in [`UPSTREAM.md`](UPSTREAM.md). When changing behavior inherited from upstream, explain whether the issue exists on the baseline and why the Ardosia fork should diverge.

Do not remove upstream attribution or rewrite the preserved ancestry to make the fork look independent of its origin.

The inherited Cargo package name remains `raknet-rust`, but this fork is Git-only for now. Do not add crates.io publishing automation, version releases, or ownership claims for the upstream package as an incidental contribution.

## Pull requests

A pull request should describe:

1. the concrete problem and solution;
2. explicit non-goals;
3. whether the change is inherited-upstream behavior or Ardosia-specific behavior;
4. API or wire/protocol compatibility impact;
5. security, abuse-control, allocation, queue, or resource-limit impact when relevant;
6. exact verification commands and results.

Update README/provenance documentation when the public support or usage contract changes.

## Security-sensitive changes

Do not use a public issue or pull request to disclose a vulnerability that has not been coordinated for public release. Follow [`SECURITY.md`](SECURITY.md).

## License

By contributing to this repository, you agree that your contribution is provided under the repository's [Apache License 2.0](LICENSE).