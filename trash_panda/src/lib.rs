use gdnative::prelude::*;

mod main_menu;
mod instructions;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<main_menu::MainMenu>();
    handle.add_class::<instructions::Instructions>();    
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);