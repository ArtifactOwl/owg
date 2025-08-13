# Architecture

```
owg/
  crates/
    owg-protocol   # messages, events, save schema, versioning
    owg-core       # deterministic utilities (rng, fixed step, hashing)
    owg-physics    # pure deterministic physics
    owg-sim        # ECS gameplay systems; command->event API
    owg-persistence# snapshots, deltas, migrations
    owg-server     # IO shell + WebSocket
  clients/
    web-client/    # thin browser test client
  fixtures/
  tests/
  ci/
```
