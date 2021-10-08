use gdnative::api::RigidBody2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Bullet {
    direction: Vector2,
    #[property]
    speed: f32,
    #[property]
    x_coeff: f32,
    #[property]
    y_coeff: f32,
    #[property]
    theta_x_coeff: f32,
    #[property]
    theta_y_coeff: f32,
    time: f32,
    #[property]
    start_theta: f32,
}

impl Bullet {
    fn new(_owner: &RigidBody2D) -> Self {
        Bullet {
            direction: Vector2::new(0.0, 0.0),
            speed: 100.0,
            x_coeff: 1.0,
            y_coeff: 1.0,
            theta_x_coeff: 1.0,
            theta_y_coeff: 1.0,
            time: 0.0,
            start_theta: 0.0,
        }
    }
}

#[methods]
impl Bullet {
    #[export]
    fn _process(&mut self, owner: &RigidBody2D, delta: f32) {
        self.time += delta;
    }

    fn _on_bullet_body_entered(&self, owner: &RigidBody2D, _body: Node) {
        owner.queue_free();
    }
}
