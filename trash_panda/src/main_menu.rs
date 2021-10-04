use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct MainMenu;

impl MainMenu {
    fn new(_owner: &Control) -> Self {
        MainMenu
    }
}

#[methods]
impl MainMenu {
    #[export]
    fn _on_start_button_pressed(&self, owner: &Control) {
        let tree = unsafe { Node::get_tree(owner).unwrap().assume_safe() };
        tree.change_scene("res://Scenes/Game.tscn")
            .expect("Problem opening or instantiating Game scene.");
    }

    #[export]
    fn _on_continue_button_pressed(&self, owner: &Control) {
        let tree = unsafe { Node::get_tree(owner).unwrap().assume_safe() };
        tree.change_scene("res://Scenes/Instructions.tscn")
            .expect("Problem opening or instantiating Instructions scene.");
    }
}
