# OWG Monorepo (Sprint A skeleton)

This is a minimal, compiling-ready scaffold for the Open-World Game (OWG).
It sets up clear module boundaries, a versioned protocol, determinism hooks,
and CI stubs to run tests across platforms.

## Quick start

```bash
# From repo root
cargo build
cargo test

# Run server stub
cargo run -p owg-server
```

See `docs/` for architecture, determinism strategy, and protocol examples.
