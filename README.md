# OWG Monorepo (Sprint A + WS)

This scaffold includes:
- Clear module boundaries (protocol/core/physics/sim/persistence/server)
- Determinism hooks (stable hashing) & tests
- Minimal **WebSocket server** (axum) that streams `Snapshot` every 100ms and handles `Ping`→`Pong`
- Tiny browser client to test WS

## Quick start

```bash
cargo build
cargo test
cargo run -p owg-server
```

Then open `clients/web-client/index.html` in your browser and click **Connect**.
You'll see periodic `Snapshot` events; click **Send Ping** to receive a `Pong`.

See `docs/` for architecture and determinism notes.
