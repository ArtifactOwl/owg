use blake3::Hasher;
use serde::Serialize;

/// Fixed step in seconds (example 60Hz).
pub const DT: f32 = 1.0 / 60.0;

/// Hash any serializable struct into a canonical blake3 hex string.
pub fn hash_state<T: Serialize>(state: &T) -> String {
    let json = serde_json::to_string(state).expect("serialize");
    let mut hasher = Hasher::new();
    hasher.update(json.as_bytes());
    hasher.finalize().to_hex().to_string()
}

/// Deterministic RNG facade (very light pseudo-rng; swap to Xoshiro/ChaCha as needed).
#[derive(Clone)]
pub struct Rng { state: u64 }
impl Rng {
    pub fn from_seed(seed: u64) -> Self { Self { state: seed } }
    #[inline] fn step(&mut self) { self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1); }
    pub fn next_u32(&mut self) -> u32 { self.step(); (self.state >> 32) as u32 }
    pub fn next_f32(&mut self) -> f32 { (self.next_u32() as f32) / (u32::MAX as f32) }
}
