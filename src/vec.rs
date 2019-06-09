use cgmath::*;
use rand::Rng;

pub fn random_vec2(w: f32, h: f32) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    Vector2::new(rng.gen_range(0., w), rng.gen_range(0., h))
}
