# Ardosia integration status

`ardosia-raknet` is a generic RakNet transport hardfork. This file records how Ardosia currently consumes the pinned transport without making Minecraft behavior part of this crate's API.

## Verified Ardosia profile

The Ardosia networking facade currently pins RakNet revision:

```text
55b57787b6715ef2a931631ef4b690e3df0651e5
```

The application configures that transport for:

- RakNet protocol `8`;
- cookie-less historical handshake behavior;
- MCPE-compatible unconnected-pong advertisement supplied as opaque application data;
- reliable-ordered connected payload delivery.

The wider stack has been exercised by a real Minecraft Windows 10 Edition Beta 0.15.10 client through login, protocol-84 bootstrap, Batch/zlib chunk streaming, `PLAYER_SPAWN`, and an actual in-world spawn. This is integration evidence for the exact pinned stack, not a claim that RakNet itself understands Minecraft.

## Boundary

Changes for game protocol 84, world/chunk data, player metadata, gameplay state, or server policy belong in `ardosia-protocol` / `ardosia-server`, not here.

This hardening pass intentionally makes **no RakNet internal code changes**. Transport behavior remains pinned to the revision above; only integration/status documentation is updated.

## Reverification

Any future change to the RakNet pin, handshake behavior, reliability/ordering implementation, fragmentation, ACK/NACK handling, or pacing requires the RakNet crate's own test gate and a real-client Ardosia smoke test before the application pin is advanced.
