# Upstream provenance

`ardosia-raknet` is an Ardosia-maintained hardfork of [`mcbe-rs/raknet-rust`](https://github.com/mcbe-rs/raknet-rust).

## Baseline

- Upstream repository: `mcbe-rs/raknet-rust`
- Upstream crate version: `0.2.0`
- Fork baseline revision: `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`
- Baseline date: 2026-03-01
- Upstream license: Apache-2.0

The Ardosia `main` branch contains that exact upstream commit with its Git ancestry preserved. Ardosia-specific maintenance is carried as ordinary commits after the baseline rather than replacing or squashing away the imported history.

At the time of the public-readiness review on 2026-08-29, the upstream repository had no commits newer than the preserved baseline above. Future upstream changes should be reviewed explicitly rather than assumed to apply cleanly to the hardfork.

## Package and distribution relationship

The inherited Cargo package/crate name remains `raknet-rust`. The upstream project owns the `raknet-rust` package published on crates.io.

Ardosia does **not** treat this hardfork as a separate crates.io release of that package. Until a deliberate publishing/versioning decision is made, consumers of the Ardosia fork should use this Git repository and pin an exact commit revision.

## Scope

The hardfork remains standalone RakNet transport infrastructure. Ardosia-specific game protocol, gameplay, world, and application behavior belong in higher layers rather than this repository.

Changes to RakNet behavior should remain evidence-driven and covered by transport-level tests. Production abuse-control defaults must not be weakened solely to satisfy localhost load-generation artifacts.

## Support relationship

The original upstream project does not endorse or support Ardosia. Issues caused by Ardosia-specific commits should be reported to this repository, not presented to upstream maintainers as upstream defects.

When a defect also reproduces on the preserved upstream baseline, maintainers may coordinate or report it upstream as appropriate while preserving attribution and the distinction between upstream behavior and Ardosia changes.

## License

The hardfork remains Apache-2.0 licensed. The upstream license text is preserved in [`LICENSE`](LICENSE); Ardosia-specific contributions are accepted under the same repository license.