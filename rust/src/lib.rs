mod inventory_system;
mod player;

use godot::{classes::ResourceLoader, prelude::*};

use crate::inventory_system::ItemLibrary;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

#[derive(GodotClass)]
#[class(singleton)]
struct GlobalSingleton {
    initialized: bool,
    item_lib: Option<Gd<ItemLibrary>>,
    base: Base<Object>,
}

#[godot_api]
impl IObject for GlobalSingleton {
    fn init(base: Base<Object>) -> Self {
        Self {
            initialized: false,
            item_lib: None,
            base: base,
        }
    }
}

#[godot_api]
impl GlobalSingleton {
    #[func]
    fn initialize(&mut self) {
        let mut loader = ResourceLoader::singleton();
        let item_lib = match loader.load("uid://jcn0k3nn4nnn") {
            Some(lib) => {
                godot_print!("initializing item library");
                let mut lib = lib.cast::<ItemLibrary>();
                match lib.bind_mut().initialize() {
                    Ok(()) => {}
                    Err(e) => godot_print!("Item library initilization error {:?}", e),
                }
                Some(lib)
            }
            None => {
                godot_print!("Could not initilize item library");
                None
            }
        };
        self.initialized = true;
        self.item_lib = item_lib;
    }

    pub fn get_item_lib(&mut self) -> Option<Gd<ItemLibrary>> {
        if !self.initialized {
            self.initialize();
        }

        match &self.item_lib {
            Some(lib) => Some(lib.clone()),
            None => None,
        }
    }
}

// #[godot_api]
// impl INode for GlobalSingleton {
//     fn ready(&mut self) {
//         godot_print!("Initilizing global nodes");
//         match &mut self.item_lib {
//             Some(item_lib) => match item_lib.bind_mut().initilize() {
//                 Err(e) => godot_print!("Item lib initilization error {:?}", e),
//                 Ok(()) => godot_print!("Item lib initilized!"),
//             },
//             None => godot_print!("Item lib not loaded!"),
//         }
//     }
// }
