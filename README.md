# OWG Monorepo (Sprint A + WebSocket)

Includes:
- Protocol/Core/Physics/Sim/Persistence/Server crates
- Determinism hooks + tests
- **Axum 0.7 WebSocket server** (TcpListener + `axum::serve`), streams `Snapshot` every 100ms, handles `Ping->Pong`
- Tiny browser client to test WS

## Run
```bash
cargo build
cargo test
cargo run -p owg-server
```
Open `clients/web-client/index.html`, click **Connect**, then **Send Ping**.
