use gdnative::api::RigidBody2D;
use gdnative::nativescript::*;
use gdnative::prelude::*;

#[derive(ToVariant, FromVariant)]
#[variant(enum = "i32")]
enum Func {
    Non,
    Lin,
    Quad,
    Sin,
    Cos,
    Tan,
}

impl Export for Func {
    type Hint = i32;
    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::I64)
    }
}

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Bullet {
    #[property]
    speed: f32,
    direction: Vector2,
    #[property]
    x_func: Func,
    #[property]
    y_func: Func,
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
            speed: 100.0,
            direction: Vector2::new(0.0, 0.0),
            x_func: Func::Non,
            y_func: Func::Non,
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
        let inc = self.speed * delta;
        self.direction = Vector2::new(self.para_func_x(inc), self.para_func_y(inc));
        let position = owner.global_position() + self.direction;
        owner.set_global_position(position);
    }

    fn para_func_x(&self, inc: f32) -> f32 {
        match self.x_func {
            Func::Non => self.x_coeff + inc,
            Func::Lin => (self.x_coeff + inc) * self.theta_x_coeff * self.time,
            Func::Quad => (self.x_coeff + inc) * self.theta_x_coeff * self.time.powi(2),
            Func::Sin => {
                let theta = self.start_theta + self.theta_x_coeff * self.time;
                (self.x_coeff + inc) * theta.sin()
            }
            Func::Cos => {
                let theta = self.start_theta + self.theta_x_coeff * self.time;
                (self.x_coeff + inc) * theta.cos()
            }
            Func::Tan => {
                let theta = self.start_theta + self.theta_x_coeff * self.time;
                (self.x_coeff + inc) * theta.tan()
            }
        }
    }

    fn para_func_y(&self, inc: f32) -> f32 {
        match self.y_func {
            Func::Non => self.y_coeff,
            Func::Lin => self.y_coeff * self.theta_y_coeff * self.time,
            Func::Quad => self.y_coeff * self.theta_y_coeff * self.time.powi(2),
            Func::Sin => {
                let theta = self.start_theta + self.theta_y_coeff * self.time;
                (self.y_coeff + inc) * theta.sin()
            }
            Func::Cos => {
                let theta = self.start_theta + self.theta_y_coeff * self.time;
                (self.y_coeff + inc) * theta.cos()
            }
            Func::Tan => {
                let theta = self.start_theta + self.theta_y_coeff * self.time;
                (self.y_coeff + inc) * theta.tan()
            }
        }
    }

    fn _on_bullet_body_entered(&self, _owner: &RigidBody2D, body: Node) {
        body.set_process(false);
    }
}
