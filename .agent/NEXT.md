# Next Work

1. Keep the exact network-consumed RakNet revision stable while higher-layer identity/inventory convergence proceeds.
2. Reconcile newer RakNet work with the consumer pin only as a bounded transport slice with the Rust 1.98 gate and any required soak/integration validation actually run.
3. Accept future work only for concrete RakNet mechanics/compatibility defects; keep game protocol, gameplay, account identity, and inventory semantics out of this crate.
4. When new transport evidence changes expected behavior, land the validated implementation here or persist the exact deferment in this queue.

Documentation centralization is complete and is not active work.
