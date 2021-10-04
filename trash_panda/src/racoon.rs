use gdnative::api::{AnimatedSprite, KinematicBody2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Racoon {
    animated_sprite: Ref<AnimatedSprite>,
    input: &'static Input,
    velocity: Vector2,
    #[property]
    speed: f32,
    animation: &'static str,
}

impl Racoon {
    fn new(_owner: &KinematicBody2D) -> Self {
        Racoon {
            animated_sprite: AnimatedSprite::new().into_shared(),
            input: Input::godot_singleton(),
            velocity: Vector2::new(0.0, 0.0),
            speed: 10.0,
            animation: "right",
        }
    }
}

#[methods]
impl Racoon {
    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.animated_sprite = unsafe {
            owner
                .get_node_as::<AnimatedSprite>("AnimatedSprite")
                .unwrap()
                .claim()
        };
    }

    #[export]
    fn _process(&mut self, owner: &KinematicBody2D, delta: f32) {
        if Input::is_action_pressed(self.input, "ui_right") {
            self.velocity.x = self.speed;
        } else if Input::is_action_pressed(self.input, "ui_left") {
            self.velocity.x = -self.speed;
        } else {
            self.velocity.x = 0.0;
        }
        if Input::is_action_pressed(self.input, "ui_down") {
            self.velocity.y = self.speed;
        } else if Input::is_action_pressed(self.input, "ui_up") {
            self.velocity.y = -self.speed;
        } else {
            self.velocity.y = 0.0;
        }
        if self.velocity.x > 0.0 {
            self.animation = "right";
        } else if self.velocity.x < 0.0 {
            self.animation = "left";
        } else {
            self.animation = "idle-right";
        }
        let position = owner.global_position()
            + Vector2::new(self.velocity.x * delta, self.velocity.y * delta);
        owner.set_global_position(position);
        let animator = unsafe { self.animated_sprite.assume_safe() };
        animator.play(self.animation, false);
    }
}
