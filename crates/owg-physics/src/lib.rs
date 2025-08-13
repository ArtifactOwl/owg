#[derive(Default, Clone)]
pub struct Integrator;
impl Integrator {
    pub fn step(&self, p: [f32;2], v: [f32;2], dt: f32) -> ([f32;2],[f32;2]) {
        let new_p = [p[0] + v[0]*dt, p[1] + v[1]*dt];
        (new_p, v)
    }
}
