use gdnative::prelude::*;

mod main_menu;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<main_menu::MainMenu>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);