use crate::config::{HEIGHT, WIDTH};
use crate::vec;
use cgmath::*;

pub struct Agent {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub target: Vector2<f32>,
    pub trail: Vec<Vector2<f32>>,
    pub speed: f32,
}

impl Agent {
    pub fn new() -> Agent {
        let position = vec::random_vec2(WIDTH, HEIGHT);
        let velocity = vec::random_vec2(WIDTH, HEIGHT);
        let target = vec::random_vec2(WIDTH, HEIGHT);
        let trail = (0..10).map(|_i| position.clone()).collect();
        let speed = 3.0;
        Agent {
            position,
            velocity,
            target,
            trail,
            speed,
        }
    }

    pub fn pick_random_target(&mut self) {
        if rand::random() {
            self.target = vec::random_vec2(WIDTH, HEIGHT);
        }
        let new_direction = Vector2::normalize(self.target - self.position);
        self.velocity = Vector2::normalize(self.velocity + new_direction) * self.speed;
    }

    pub fn update(&mut self) {
        let mut prev_position = self.position;
        self.position += self.velocity;
        for position in self.trail.iter_mut() {
            let temp = *position;
            *position = prev_position;
            prev_position = temp;
        }
    }
}
