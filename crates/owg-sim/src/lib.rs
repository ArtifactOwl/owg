use anyhow::Result;
use owg_core::{DT, hash_state};
use owg_physics::Integrator;
use owg_protocol::{Cmd, Evt, State, Envelope, Kind, SchemaVersion};

pub struct Sim {
    integ: Integrator,
    pub state: owg_protocol::State,
    tick: u64,
}

impl Sim {
    pub fn new(seed: &str) -> Self {
        let state = owg_protocol::State {
            world: owg_protocol::World { seed: seed.to_string(), time: 0 },
            ..Default::default()
        };
        Self { integ: Integrator::default(), state, tick: 0 }
    }

    pub fn apply(&mut self, cmd: Cmd) -> Vec<Evt> {
        match cmd {
            Cmd::Ping { nonce } => vec![Evt::Pong { nonce, rtt_ms: 0 }],
            _ => vec![],
        }
    }

    pub fn step(&mut self) {
        // Example: integrate all entity poses naively
        for e in &mut self.state.entities {
            let (np, nv) = self.integ.step(e.pose.p, e.pose.v, DT);
            e.pose.p = np;
            e.pose.v = nv;
        }
        self.tick += 1;
        self.state.world.time = self.tick;
    }

    pub fn state_hash(&self) -> String { hash_state(&self.state) }
    pub fn tick(&self) -> u64 { self.tick }

    pub fn snapshot_envelope(&self) -> Envelope<Evt> {
        Envelope {
            kind: Kind::Evt,
            schema: SchemaVersion::V0_1,
            t: self.tick,
            id: None,
            body: Evt::Snapshot { full: true, state: self.state.clone() }
        }
    }
}

pub fn run_headless_example() -> Result<()> {
    let mut sim = Sim::new("W-2025-08-A");
    for _ in 0..3 { sim.step(); }
    let snap = sim.snapshot_envelope();
    println!("Snapshot@{} hash={}", snap.t, hash_state(&sim.state));
    Ok(())
}
