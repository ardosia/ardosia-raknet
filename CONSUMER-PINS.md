# Consumer pins and repository HEAD

`ardosia-raknet` is a reusable pre-release hardfork. Its repository `main` branch and the revisions consumed by downstream projects are intentionally different concepts.

## Current Ardosia consumer pin

At the 2026-09-13 Ardosia maintenance checkpoint, `ardosia-network` pins:

```text
55b57787b6715ef2a931631ef4b690e3df0651e5
```

That exact revision is the transport version the active Network facade records as its verified dependency. Newer commits on this repository's `main` branch do **not** automatically become part of the Ardosia application stack.

## Pin movement policy

A downstream pin should move only as an explicit transport integration change. Reviewers should identify:

- the old and new exact SHAs;
- transport behavior changed between them;
- relevant tests/CI or compatibility evidence;
- whether protocol-version, handshake, abuse-control, pacing, reliability, or runtime behavior changed;
- any downstream configuration changes required by the new revision.

Do not replace an exact SHA with a moving branch for convenience.

## Repository scope

This file records consumption state only. It does not make the hardfork Minecraft-specific: game packet semantics, gameplay/world state, and Ardosia application policy remain outside this repository.

See `README.md` for project scope and `UPSTREAM.md` for fork provenance.
