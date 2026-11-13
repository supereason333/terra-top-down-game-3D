mod item;

use std::collections::HashMap;

use godot::classes::Node3D;
use godot::obj::Bounds;
use godot::obj::bounds::DeclUser;
use godot::prelude::*;

use crate::inventory_system::item::ItemInstance;

#[derive(Debug)]
pub enum ItemLibraryErr {
    InitilizeError,
    NoSuchItem,
    NoScene,
    WrongNodeType,
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

    pub fn get_item_node<I>(
        &self,
        id: &StringName,
        instance: I,
    ) -> Result<Gd<I::Node>, ItemLibraryErr>
    where
        I: ItemInstanceData,
    {
        let item = self.get_item(id).ok_or(ItemLibraryErr::NoSuchItem)?;

        let scene = item
            .bind()
            .item_scene
            .clone()
            .ok_or(ItemLibraryErr::NoScene)?;

        let node = scene
            .instantiate_as::<Node3D>()
            .try_cast::<I::Node>()
            .map_err(|_| ItemLibraryErr::WrongNodeType)?;

        instance.load_into_item(node.clone());

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
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemDefinitionResource {
    //
}

impl ItemDefinitionResource {}

// every item has one of these

pub trait ItemInstanceData: Sized {
    type ItemNode: GodotClass + ItemInstance + Inherits<Node3D> + Bounds<Declarer = DeclUser>;

    // constructor for default data
    fn new() -> Self;

    // constructs from an item's data
    fn new_from_item(item: Gd<Self::ItemNode>) -> Option<Self>;

    // when item is equipped, load data into item
    fn load_into_item(&self, item: Gd<Self::ItemNode>);

    // when item is unequipped, take data from item
    fn take_from_item(&mut self, item: Gd<Self::ItemNode>);

    //     // constructs from an item's data
    //     fn new_from_item<N>(item: Gd<N>) -> Option<Self>
    //     where
    //         Self: Sized,
    //         N: GodotClass + ItemInstance + Inherits<Node3D> + Bounds<Declarer = DeclUser>;
    //
    //     // when item is equipped, load data into item
    //     fn load_into_item<N>(&self, item: Gd<N>)
    //     where
    //         N: GodotClass + ItemInstance + Inherits<Node3D> + Bounds<Declarer = DeclUser>;
    //
    //     // when item is unequipped, take data from item
    //     fn take_from_item<N>(&mut self, item: Gd<N>)
    //     where
    //         N: GodotClass + ItemInstance + Inherits<Node3D> + Bounds<Declarer = DeclUser>;
}
