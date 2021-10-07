use gdnative::api::{Node2D, PackedScene, ResourceLoader, RigidBody2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct BulletManager {
    #[property]
    num_bullets: i32,
    #[property]
    time_delay: f32,
    time: f32,
    bullets: Vec<Ref<RigidBody2D>>,
}

impl BulletManager {
    fn new(_owner: &Node2D) -> Self {
        BulletManager {
            num_bullets: 0,
            time_delay: 0.0,
            time: 0.0,
            bullets: Vec::new(),
        }
    }
}

#[methods]
impl BulletManager {
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        self.create_bullet_round();
    }

    fn create_bullet_round(&mut self) {
        let mut num = 0;
        let resource = ResourceLoader::godot_singleton();
        while num < self.num_bullets {
            unsafe {
                self.bullets.push(
                    ResourceLoader::load(
                        resource,
                        "res://Scenes/Bullet.tscn",
                        "RigidBody2D",
                        false,
                    )
                    .unwrap()
                    .assume_safe()
                    .cast::<PackedScene>()
                    .unwrap()
                    .instance(0)
                    .unwrap()
                    .assume_safe()
                    .cast::<RigidBody2D>()
                    .unwrap()
                    .claim(),
                )
            }
            num += 1;
        }
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, delta: f32) {
        self.time += delta;
        if self.time / self.time_delay >= 1.0 {
            self.time = 0.0;
            self.create_bullet_round();
        }
    }
}
