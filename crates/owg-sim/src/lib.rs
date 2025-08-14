
use anyhow::Result;
use owg_core::{DT, hash_state};
use owg_physics::Integrator;
use owg_protocol::{Cmd, Evt, Envelope, Kind, SchemaVersion};

pub struct Sim {
    integ: Integrator,
    pub state: owg_protocol::State,
    tick: u64,
    proj_counter: u64,
}

impl Sim {
    pub fn new(seed: &str) -> Self {
        let state = owg_protocol::State {
            world: owg_protocol::World { seed: seed.to_string(), time: 0 },
            ..Default::default()
        };
        Self { integ: Integrator::default(), state, tick: 0, proj_counter: 0 }
    }

    pub fn apply(&mut self, cmd: Cmd) -> Vec<Evt> {
        match cmd {
            Cmd::Ping { nonce } => vec![Evt::Pong { nonce, rtt_ms: 0 }],
            Cmd::Mine { entity_id, target } => {
                vec![Evt::Mined { miner_id: entity_id, node_id: target, yields: vec![("FeOre".to_string(), 1)] }]
            }
            Cmd::Fire { entity_id, weapon, aim } => {
                // Normalize aim
                let mag = (aim[0]*aim[0] + aim[1]*aim[1]).sqrt().max(1e-6);
                let dir = [aim[0]/mag, aim[1]/mag];
                let speed = 5.0;
                let vel = [dir[0]*speed, dir[1]*speed];

                // Create projectile record
                let pid = format!("p{}_{}", self.tick, self.proj_counter);
                self.proj_counter += 1;
                let proj = owg_protocol::Projectile {
                    id: pid.clone(),
                    pose: owg_protocol::Pose { p: [0.0, 0.0], v: vel, theta: 0.0, omega: 0.0 },
                    ttl: 60,
                    owner: entity_id.clone(),
                    weapon: weapon.clone(),
                };
                self.state.projectiles.push(proj);

                // Mirror as an entity so Delta updates can show movement
                let ent = owg_protocol::Entity {
                    id: format!("e_proj_{}", pid),
                    archetype: format!("projectile/{}", weapon),
                    pose: owg_protocol::Pose { p: [0.0, 0.0], v: vel, theta: 0.0, omega: 0.0 },
                    physics: owg_protocol::Physics { mass: 1.0, radius: 0.2 },
                    inventory: owg_protocol::Inventory { slots: vec![] },
                    owner: None,
                };
                self.state.entities.push(ent);

                vec![] // movement will be visible via Delta
            }
            _ => vec![],
        }
    }

    pub fn step(&mut self) {
        // Move entities
        for e in &mut self.state.entities {
            let (np, nv) = self.integ.step(e.pose.p, e.pose.v, DT);
            e.pose.p = np;
            e.pose.v = nv;
        }
        // Move projectiles & remove expired, and sync their mirror entities
        for p in &mut self.state.projectiles {
            let (np, nv) = self.integ.step(p.pose.p, p.pose.v, DT);
            p.pose.p = np;
            p.pose.v = nv;
            if p.ttl > 0 { p.ttl -= 1; }
        }
        // Remove expired projectiles and their mirror entities
        let expired_ids: Vec<String> = self.state.projectiles.iter().filter(|p| p.ttl == 0).map(|p| p.id.clone()).collect();
        if !expired_ids.is_empty() {
            self.state.projectiles.retain(|p| p.ttl > 0);
            self.state.entities.retain(|e| {
                if let Some(pid) = e.id.strip_prefix("e_proj_") {
                    !expired_ids.iter().any(|x| x == pid)
                } else { true }
            });
        } else {
            // Sync mirror entity positions
            for p in &self.state.projectiles {
                let eid = format!("e_proj_{}", p.id);
                if let Some(e) = self.state.entities.iter_mut().find(|e| e.id == eid) {
                    e.pose = p.pose.clone();
                }
            }
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
    println!("Snapshot@{} hash={}", sim.tick, hash_state(&self.state));
    Ok(())
}
