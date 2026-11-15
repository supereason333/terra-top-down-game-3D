mod entities;
mod inventory_system;
mod other_stuff;
mod player;
mod projectile_system;
mod world;

use godot::{classes::ResourceLoader, prelude::*};

use crate::{
    inventory_system::ItemLibrary,
    player::player_node::Player,
    projectile_system::ProjectileLibrary,
    world::world_node::{ProjectilesNode, WorldNode},
};

struct RustExtension;

const ITEM_LIBRARY_UID: &str = "uid://jcn0k3nn4nnn";
const PROJECTILE_LIBRARY_UID: &str = "uid://dsanjnuyf4s5l";

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

#[derive(GodotClass)]
#[class(singleton)]
struct GlobalSingleton {
    initialized: bool,
    item_lib: Option<Gd<ItemLibrary>>,
    projectile_lib: Option<Gd<ProjectileLibrary>>,

    world_node: Option<Gd<WorldNode>>,
    projectiles_node: Option<Gd<ProjectilesNode>>,
    player_node: Option<Gd<Player>>,

    base: Base<Object>,
}

#[godot_api]
impl IObject for GlobalSingleton {
    fn init(base: Base<Object>) -> Self {
        Self {
            initialized: false,
            item_lib: None,
            projectile_lib: None,
            world_node: None,
            projectiles_node: None,
            player_node: None,
            base: base,
        }
    }
}

#[godot_api]
impl GlobalSingleton {
    #[func]
    fn initialize(&mut self) {
        let mut loader = ResourceLoader::singleton();

        // item lib init
        let item_lib = match loader.load(ITEM_LIBRARY_UID) {
            Some(lib) => {
                godot_print!("Initializing item library");
                let mut lib = lib.cast::<ItemLibrary>();
                match lib.bind_mut().initialize() {
                    Ok(()) => {}
                    Err(e) => godot_print!("Item library initilization error {:?}", e),
                }
                Some(lib)
            }
            None => {
                godot_print!("Could not load item library");
                None
            }
        };
        self.item_lib = item_lib;

        let projectile_lib = match loader.load(PROJECTILE_LIBRARY_UID) {
            Some(lib) => {
                godot_print!("Initializing projectile library");
                let mut lib = lib.cast::<ProjectileLibrary>();
                match lib.bind_mut().initialize() {
                    Ok(()) => {}
                    Err(e) => godot_print!("Projectile library initilization error {:?}", e),
                }
                Some(lib)
            }
            None => {
                godot_print!("Could not load projectile library");
                None
            }
        };
        self.projectile_lib = projectile_lib;

        self.initialized = true;
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

    pub fn get_projectile_lib(&mut self) -> Option<Gd<ProjectileLibrary>> {
        if !self.initialized {
            self.initialize();
        }

        match &self.projectile_lib {
            Some(lib) => Some(lib.clone()),
            None => None,
        }
    }

    pub fn set_world_node(&mut self, world: Gd<WorldNode>) {
        world
            .clone()
            .upcast::<Node>()
            .signals()
            .tree_exited()
            .connect_other(self, Self::erase_world_node);
        self.world_node = Some(world);
    }

    pub fn erase_world_node(&mut self) {
        self.world_node = None;
    }

    pub fn get_world_node(&mut self) -> Option<Gd<WorldNode>> {
        match self.world_node.clone() {
            Some(world) => {
                if world.is_instance_valid() {
                    Some(world)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn set_projectiles_node(&mut self, node: Gd<ProjectilesNode>) {
        node.clone()
            .upcast::<Node>()
            .signals()
            .tree_exited()
            .connect_other(self, Self::erase_world_node);
        self.projectiles_node = Some(node);
    }

    pub fn erase_projectiles_node(&mut self) {
        self.projectiles_node = None;
    }

    pub fn get_projectiles_node(&mut self) -> Option<Gd<ProjectilesNode>> {
        match self.projectiles_node.clone() {
            Some(node) => {
                if node.is_instance_valid() {
                    Some(node)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn set_player_node(&mut self, node: Gd<Player>) {
        node.clone()
            .upcast::<Node>()
            .signals()
            .tree_exited()
            .connect_other(self, Self::erase_world_node);
        self.player_node = Some(node);
    }

    pub fn erase_player_node(&mut self) {
        self.player_node = None;
    }

    pub fn get_player_node(&mut self) -> Option<Gd<Player>> {
        match self.player_node.clone() {
            Some(node) => {
                if node.is_instance_valid() {
                    Some(node)
                } else {
                    None
                }
            }
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
