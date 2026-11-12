mod inventory_system;
mod player;

use godot::prelude::*;

use crate::inventory_system::ItemLibrary;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct GlobalNode {
    #[export]
    item_lib: Option<Gd<ItemLibrary>>,
}

#[godot_api]
impl INode for GlobalNode {
    fn ready(&mut self) {
        godot_print!("Initilizing global nodes");
        match &mut self.item_lib {
            Some(item_lib) => match item_lib.bind_mut().initilize() {
                Err(e) => godot_print!("Item lib initilization error {:?}", e),
                Ok(()) => godot_print!("Item lib initilized!"),
            },
            None => godot_print!("Item lib not loaded!"),
        }
    }
}
