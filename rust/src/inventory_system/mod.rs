mod item;

use std::collections::HashMap;

use godot::obj::Bounds;
use godot::obj::bounds::DeclUser;
use godot::{
    classes::{Node3D, class_macros::private::virtuals::Node3D},
    prelude::*,
};

use crate::inventory_system::item::ItemInstance;

#[derive(Debug)]
pub enum ItemLibraryErr {
    InitilizeError,
    NoSuchItem,
    NoScene,
}

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct ItemLibrary {
    #[export]
    lib: Array<Gd<ItemDefinitionResource>>,
    initilized: bool,

    lookup: HashMap<StringName, Gd<ItemDefinitionResource>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemLibrary {
    fn init(base: Base<Resource>) -> Self {
        Self {
            lib: Array::new(),
            initilized: false,

            lookup: HashMap::new(),

            base: base,
        }
    }
}

#[godot_api]
pub impl ItemLibrary {
    /// initilises the library, checks for errors and such
    pub fn initilize(&mut self) -> Result<(), ItemLibraryErr> {
        let seen: HashMap<StringName, ()> = HashMap::new();
        for item in self.lib.iter_shared() {
            let id = item.bind().id.clone();
            if seen.get(&id) == Some(&()) {
                godot_error!("Two items share the same name! {id}");
                continue;
            }
            self.lookup.insert(id, item);
        }

        self.initilized = true;
        Ok(())
    }

    pub fn get_item(&self, id: &StringName) -> Option<Gd<ItemDefinitionResource>> {
        match self.lookup.get(id) {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }

    pub fn get_item_node<N, I>(
        &self,
        id: &StringName,
        instance: Gd<ItemInstanceResource>,
    ) -> Result<Gd<N>, ItemLibraryErr>
    where
        N: ItemInstance,
        N: GodotClass,
        N: Inherits<Node3D>,
        N: Bounds<Declarer = DeclUser>,
    {
        let item = match self.get_item(id) {
            Some(item) => item,
            None => return Err(ItemLibraryErr::NoSuchItem),
        };

        let mut node: Gd<N> = match item.bind().item_scene.clone() {
            Some(scene) => scene.instantiate_as::<Node3D>().cast::<N>(),
            None => return Err(ItemLibraryErr::NoScene),
        };

        node.bind_mut().set_instance(instance);

        Ok(node)
    }
}

// used in library
#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct ItemDefinitionResource {
    #[export]
    display_name: GString,
    #[export]
    id: StringName,
    #[export]
    pub item_scene: Option<Gd<PackedScene>>,
    #[export]
    default_instance_resource: Option<Gd<ItemInstanceResource>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemDefinitionResource {
    //
}

impl ItemDefinitionResource {}

// every item has one of these
#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct ItemInstanceResource {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemInstanceResource {
    //
}
