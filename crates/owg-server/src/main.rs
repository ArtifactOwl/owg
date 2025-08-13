
use std::{collections::HashMap, env, fs::File, io::{BufRead, BufReader}, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    extract::State,
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::{broadcast, Mutex}};
use tracing::{info, warn};

use owg_sim::Sim;
use owg_protocol::{Envelope, Kind, SchemaVersion, Cmd, Evt, Entity, State as GameState};

#[derive(Clone)]
struct AppState {
    sim: Arc<Mutex<Sim>>,
    tx: broadcast::Sender<String>, // JSON payloads to clients
    replay: Arc<Option<Vec<(u64, Cmd)>>>, // (tick, cmd)
    snapshot_interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let mut args = env::args().skip(1);
    let mut replay: Option<Vec<(u64, Cmd)>> = None;
    let mut snapshot_interval: u64 = 10;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--replay" => {
                if let Some(path) = args.next() {
                    replay = Some(load_replay(&path)?);
                    info!("loaded replay: {} commands", replay.as_ref().unwrap().len());
                }
            }
            "--snapshot-interval" => {
                if let Some(v) = args.next() {
                    snapshot_interval = v.parse().unwrap_or(10);
                }
            }
            _ => {}
        }
    }

    let sim = Arc::new(Mutex::new(Sim::new("W-2025-08-A")));
    let (tx, _rx) = broadcast::channel::<String>(256);
    let state = AppState { sim, tx, replay: Arc::new(replay), snapshot_interval };

    spawn_ticker(state.clone());

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!(%addr, "owg-server listening");
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .try_init();
}

fn load_replay(path: &str) -> Result<Vec<(u64, Cmd)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        #[derive(serde::Deserialize)]
        struct Line { t: u64, cmd: serde_json::Value }
        let l: Line = serde_json::from_str(&line)?;
        if let Some(cmd) = parse_cmd_value(l.cmd) {
            out.push((l.t, cmd));
        }
    }
    out.sort_by_key(|(t, _)| *t);
    Ok(out)
}

fn parse_cmd_value(v: serde_json::Value) -> Option<Cmd> {
    // Accept either { "type":"Ping", ... } or { "Ping": {...} }
    if let Ok(cmd) = serde_json::from_value::<Cmd>(v.clone()) {
        return Some(cmd);
    }
    if let serde_json::Value::Object(map) = v {
        if map.len() == 1 {
            let (k, inner) = map.into_iter().next().unwrap();
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), serde_json::Value::String(k));
            if let serde_json::Value::Object(inner_map) = inner {
                for (ik, iv) in inner_map {
                    obj.insert(ik, iv);
                }
            }
            let tagged = serde_json::Value::Object(obj);
            if let Ok(cmd) = serde_json::from_value::<Cmd>(tagged) { return Some(cmd); }
        }
    }
    None
}

fn diff_entities(prev: &GameState, curr: &GameState) -> (Vec<Entity>, Vec<Entity>, Vec<String>) {
    let mut prev_map: HashMap<&str, &Entity> = HashMap::new();
    for e in &prev.entities { prev_map.insert(e.id.as_str(), e); }
    let mut curr_map: HashMap<&str, &Entity> = HashMap::new();
    for e in &curr.entities { curr_map.insert(e.id.as_str(), e); }

    let mut adds = Vec::new();
    let mut updates = Vec::new();
    let mut removes = Vec::new();

    for (id, e) in &curr_map {
        match prev_map.get(id) {
            None => adds.push((*e).clone()),
            Some(pe) => if *pe != *e { updates.push((*e).clone()); }
        }
    }
    for (id, _) in &prev_map {
        if !curr_map.contains_key(id) { removes.push((*id).to_string()); }
    }
    (adds, updates, removes)
}

fn spawn_ticker(state: AppState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        let mut last_state = {
            let sim = state.sim.lock().await;
            sim.state.clone()
        };
        loop {
            interval.tick().await;

            // Apply replay commands scheduled for this tick
            if let Some(ref replay) = *state.replay {
                let t = { state.sim.lock().await.tick() + 1 }; // will increment this step
                for (_t, cmd) in replay.iter().filter(|(rt, _)| *rt == t) {
                    let _ = state.sim.lock().await.apply(cmd.clone());
                }
            }

            // Step sim
            {
                let mut sim = state.sim.lock().await;
                sim.step();
                let now = sim.state.clone();

                // Snapshot every N ticks; otherwise Delta
                if sim.tick() % state.snapshot_interval == 0 {
                    let env = sim.snapshot_envelope();
                    if let Ok(json) = serde_json::to_string(&env) { let _ = state.tx.send(json); }
                } else {
                    let (adds, updates, removes) = diff_entities(&last_state, &now);
                    let env = Envelope {
                        kind: Kind::Evt,
                        schema: SchemaVersion::V0_1,
                        t: sim.tick(),
                        id: None,
                        body: Evt::Delta { adds, updates, removes },
                    };
                    if let Ok(json) = serde_json::to_string(&env) { let _ = state.tx.send(json); }
                }

                last_state = now;
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

    // Forward tick events to this client
    let mut rx = state.tx.subscribe();
    tokio::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if sender.send(Message::Text(text)).await.is_err() { break; }
        }
    });

    // Handle incoming client commands
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(txt) = msg {
            match serde_json::from_str::<Envelope<Cmd>>(&txt) {
                Ok(env) => {
                    let events = { state.sim.lock().await.apply(env.body) };
                    for evt in events {
                        let t = { state.sim.lock().await.tick() };
                        let env_evt = Envelope {
                            kind: Kind::Evt,
                            schema: SchemaVersion::V0_1,
                            t,
                            id: None,
                            body: evt,
                        };
                        if let Ok(json) = serde_json::to_string(&env_evt) { let _ = state.tx.send(json); }
                    }
                }
                Err(e) => warn!("ignored non-cmd message: {}", e),
            }
        }
    }
    info!("websocket disconnected");
}
