## Summary

Describe the concrete problem and the change that solves it.

## Scope and non-goals

- In scope:
- Explicitly out of scope:

## Upstream and provenance impact

- Does this change behavior inherited from `mcbe-rs/raknet-rust`?
- If yes, does the issue reproduce on the preserved upstream baseline `3edfb4170e6cb5aeed992b09b50176fb7e5b6079`?
- Does this change attribution, package metadata, or the documented fork relationship?

## API and compatibility impact

Describe any changes to:

- public Rust API;
- RakNet wire behavior or protocol-version handling;
- runtime/session/reliability behavior;
- migration or pinning requirements for consumers.

Write `None` when there is no impact.

## Security and abuse-control impact

Describe any effect on parsing bounds, allocations, queues, rate limits, IP blocking, processing budgets, fragmentation/reassembly, retransmission, or denial-of-service exposure.

Do not weaken production abuse-control defaults solely to improve localhost benchmark results.

## Verification

List exact commands and results. The normal gate is:

```bash
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --all-targets --all-features -- -D warnings
cargo +1.98.0 test --all-targets
git diff --check
```

For transport/runtime changes, include focused regression/integration evidence and run the short soak when relevant:

```bash
cargo run --quiet --example raknet_soak -- --sessions=64 --ticks=120 --payload-bytes=120
```

## Checklist

- [ ] The change stays generic to RakNet and does not add game/application policy.
- [ ] Tests or concrete evidence cover behavior changes.
- [ ] Documentation is updated when the public API/support contract changes.
- [ ] No credentials, private infrastructure details, or uncoordinated vulnerability details are included.
- [ ] No crates.io publishing/version ownership change is introduced incidentally.
