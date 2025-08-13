
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaVersion { pub major: u16, pub minor: u16 }
impl SchemaVersion { pub const V0_1: SchemaVersion = SchemaVersion{ major:0, minor:1 }; }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "cmd", alias = "Cmd")]
    Cmd,
    #[serde(rename = "evt", alias = "Evt")]
    Evt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub kind: Kind,
    pub schema: SchemaVersion,
    pub t: u64,
    pub id: Option<Uuid>,
    pub body: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum Cmd {
    Ping { nonce: String },
    Move { entity_id: String, axis: [f32;2], thrust: f32 },
    Fire { entity_id: String, weapon: String, aim: [f32;2] },
    Mine { entity_id: String, target: String },
    Craft { entity_id: String, recipe_id: String, inputs: Vec<(String,i32)> },
    UseItem { entity_id: String, slot: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum Evt {
    Pong { nonce: String, rtt_ms: u32 },
    Snapshot { full: bool, state: State },
    Delta { adds: Vec<Entity>, updates: Vec<Entity>, removes: Vec<String> },
    Mined { miner_id: String, node_id: String, yields: Vec<(String,i32)> },
    CraftResult { entity_id: String, ok: bool, outputs: Vec<(String,i32)> },
    Desync { client_hash: String, server_hash: String, at_tick: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct State {
    pub world: World,
    pub entities: Vec<Entity>,
    pub projectiles: Vec<Projectile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct World { pub seed: String, pub time: u64 }

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Pose { pub p: [f32;2], pub v: [f32;2], pub theta: f32, pub omega: f32 }

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Entity {
    pub id: String,
    pub archetype: String,
    pub pose: Pose,
    pub physics: Physics,
    pub inventory: Inventory,
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Physics { pub mass: f32, pub radius: f32 }

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Inventory { pub slots: Vec<(String,i32)> }

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Projectile {
    pub id: String,
    pub pose: Pose,
    pub ttl: u16,
    pub owner: String,
    pub weapon: String
}
