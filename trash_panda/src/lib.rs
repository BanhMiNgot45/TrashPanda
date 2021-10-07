use gdnative::prelude::*;

mod bullet;
mod bullet_manager;
mod instructions;
mod main_menu;
mod racoon;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<main_menu::MainMenu>();
    handle.add_class::<instructions::Instructions>();
    handle.add_class::<racoon::Racoon>();
    handle.add_class::<bullet::Bullet>();
    handle.add_class::<bullet_manager::BulletManager>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
