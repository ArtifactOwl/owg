# Architecture

```
owg/
  crates/
    owg-protocol   # messages, events, save schema, versioning
    owg-core       # deterministic utilities (rng, fixed step, hashing)
    owg-physics    # pure deterministic physics
    owg-sim        # ECS gameplay systems; command->event API
    owg-persistence# snapshots, deltas, migrations
    owg-server     # IO shell, tick loop, replay capture, WebSocket
  clients/
    web-client/    # thin browser client for WS testing
  fixtures/        # sample content and tiny test worlds
  tests/           # protocol & determinism tests
  ci/              # CI workflows
```
Module boundaries: clients/server use **owg-sim** only via protocol commands/events.
No direct game logic in clients or server.
