# Next Work

RakNet runtime work is **paused** while repository hygiene is active.

1. Keep the exact network-consumed RakNet revision stable during cleanup.
2. Treat `docs-runtime-status` and the merged workflow branch as branch-hygiene items, not active engineering work.
3. After branch cleanup, verify `STATE.md` contains only live pin/branch facts.

## Parked runtime work
When explicitly resumed, accept only concrete RakNet mechanics/compatibility work and validate bounded transport changes with the Rust 1.98 gate plus any required soak/integration checks before moving the network consumer pin.
