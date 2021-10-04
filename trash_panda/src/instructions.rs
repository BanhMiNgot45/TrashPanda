use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Instructions;

impl Instructions {
    fn new(_owner: &Control) -> Self {
        Instructions
    }
}

#[methods]
impl Instructions {
    #[export]
    fn _on_back_button_pressed(&self, owner: &Control) {
        let tree = unsafe { Node::get_tree(owner).unwrap().assume_safe() };
        tree.change_scene("res://Scenes/MainMenu.tscn")
            .expect("Problem opening or instantiating Main Menu scene.");
    }
}
