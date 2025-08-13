use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    extract::State,
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{StreamExt, SinkExt};
use tokio::sync::{broadcast, Mutex};
use tracing::{info, warn};

use owg_sim::{Sim};
use owg_protocol::{Envelope, Kind, SchemaVersion, Cmd, Evt};

#[derive(Clone)]
struct AppState {
    sim: Arc<Mutex<Sim>>,
    tx: broadcast::Sender<String>, // JSON payloads broadcast to clients
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let sim = Arc::new(Mutex::new(Sim::new("W-2025-08-A")));
    let (tx, _rx) = broadcast::channel::<String>(128);
    let state = AppState { sim, tx };

    // Spawn ticker that steps the sim and broadcasts a Snapshot every 100ms
    spawn_ticker(state.clone());

    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    info!("owg-server listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .try_init();
}

fn spawn_ticker(state: AppState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;

            // Step the sim
            {
                let mut sim = state.sim.lock().await;
                sim.step();
                let env = sim.snapshot_envelope();
                if let Ok(json) = serde_json::to_string(&env) {
                    let _ = state.tx.send(json);
                }
            }
        }
    });
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

async fn handle_socket(state: AppState, socket: WebSocket) {
    info!("websocket connected");
    let (mut sender, mut receiver) = socket.split();

    // Send initial snapshot immediately
    if let Ok(json) = {
        let sim = state.sim.lock().await;
        serde_json::to_string(&sim.snapshot_envelope())
    } {
        if let Err(e) = sender.send(Message::Text(json)).await {
            warn!("failed to send initial snapshot: {}", e);
            return;
        }
    }

    // Subscribe to broadcast channel to forward snapshots/deltas
    let mut rx = state.tx.subscribe();
    let mut sender_clone = sender.clone();
    tokio::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if sender_clone.send(Message::Text(text)).await.is_err() {
                break; // client disconnected
            }
        }
    });

    // Process incoming client messages (Cmd envelopes in JSON)
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(txt) = msg {
            match serde_json::from_str::<Envelope<Cmd>>(&txt) {
                Ok(env) => {
                    let events = {
                        let mut sim = state.sim.lock().await;
                        sim.apply(env.body)
                    };
                    // Wrap events into envelopes and broadcast (simple for now)
                    for evt in events {
                        let t = {
                            let sim = state.sim.lock().await;
                            sim.tick()
                        };
                        let env_evt = Envelope {
                            kind: Kind::Evt,
                            schema: SchemaVersion::V0_1,
                            t,
                            id: None,
                            body: evt,
                        };
                        if let Ok(json) = serde_json::to_string(&env_evt) {
                            let _ = state.tx.send(json);
                        }
                    }
                }
                Err(e) => warn!("ignored non-cmd message: {}", e),
            }
        }
    }
    info!("websocket disconnected");
}
