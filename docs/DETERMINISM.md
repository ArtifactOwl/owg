# Determinism Plan (Sprint A)

- Fixed-step lockstep (server authority), clients predict and reconcile.
- Stable RNG facade seeded by world seed; one stream per subsystem/entity.
- Strict ordering: systems iterate in deterministic order; entities sorted by id.
- Math: avoid nondeterministic transcedentals; consider fixed-point for integration.
- Canonical state hashing with BLAKE3 over sorted minimal snapshot.
- CI runs identical replays on Windows/Linux/macOS.

See `tests/determinism` for hash checks and `fixtures/test_worlds` for inputs.
