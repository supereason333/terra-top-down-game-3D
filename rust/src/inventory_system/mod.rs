pub mod item;

use std::str::FromStr;
use std::{any::Any, collections::HashMap};

use godot::classes::Node3D;
use godot::obj::Bounds;
use godot::obj::bounds::DeclUser;
use godot::prelude::*;

use crate::inventory_system::item::{BasicGunInstanceData, ItemInstance, TestItemInstanceData};

type Factory = fn() -> Box<dyn ItemInstanceDataObject>;

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

    data_registry: ItemDataRegistry,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemLibrary {
    fn init(base: Base<Resource>) -> Self {
        Self {
            lib: Array::new(),
            initilized: false,

            lookup: HashMap::new(),

            data_registry: ItemDataRegistry::default(),

            base: base,
        }
    }
}

impl ItemLibrary {
    /// initilises the library, checks for errors and such
    pub fn initialize(&mut self) -> Result<(), ItemLibraryErr> {
        // first do godot linked setup
        for item in self.lib.iter_shared() {
            let id = item.bind().id.clone();
            if self.lookup.get(&id).is_some() {
                godot_print!("Two items share the same name! {id}");
                continue;
            }
            self.lookup.insert(id, item);
        }

        let registered = self.lookup.len();
        let total = self.lib.len();

        godot_print!("Registered {registered} items out of {total}");

        // does the data registery set up
        register_all(&mut self.data_registry);

        let data_registered = self.data_registry.factories.len();

        godot_print!("Registered {data_registered} data factories");

        if data_registered > registered {
            let dif = data_registered - registered;
            godot_print!("{dif} items not registered");

            for &id in self.data_registry.factories.keys() {
                if !self.lookup.contains_key(&StringName::from_str(id).unwrap()) {
                    godot_print!("Item {id} not found in library");
                }
            }
        } else if registered > data_registered {
            let dif = registered - data_registered;
            godot_print!("{dif} items not registered in data registery");

            for id in self.lookup.keys() {
                if !self
                    .data_registry
                    .factories
                    .contains_key(id.to_string().as_str())
                {
                    godot_print!("Item {id} not found in data registery");
                }
            }
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

    pub fn get_item_node(
        &self,
        ins_data: &dyn ItemInstanceDataObject,
    ) -> Result<Gd<Node3D>, ItemLibraryErr> {
        let item = self
            .get_item(&StringName::from(ins_data.item_id()))
            .ok_or(ItemLibraryErr::NoSuchItem)?;

        let scene = item
            .bind()
            .item_scene
            .clone()
            .ok_or(ItemLibraryErr::NoScene)?;

        let node = scene.instantiate_as::<Node3D>();

        if let Err(e) = ins_data.load_into(node.clone()) {
            node.free();
            return Err(e);
        }

        Ok(node)
    }

    pub fn get_item_data_str(&mut self, id: &str) -> Option<Box<dyn ItemInstanceDataObject>> {
        self.data_registry.create(id)
    }

    pub fn get_item_data(&mut self, id: &StringName) -> Option<Box<dyn ItemInstanceDataObject>> {
        self.data_registry.create(&id.to_string().as_str())
    }
}

#[derive(Default)]
pub struct ItemDataRegistry {
    factories: HashMap<&'static str, Factory>,
}

impl ItemDataRegistry {
    pub fn register<I: ItemInstanceData + 'static>(&mut self) {
        self.factories.insert(I::ITEM_ID, || Box::new(I::new()));
    }

    pub fn create(&self, id: &str) -> Option<Box<dyn ItemInstanceDataObject>> {
        self.factories.get(id).map(|f| f())
    }
}

// registers all itmes, DONT FORGET: ADD HERE AFTER ADDING TO GODOT LIBRARY
fn register_all(reg: &mut ItemDataRegistry) {
    reg.register::<TestItemInstanceData>();
    reg.register::<BasicGunInstanceData>();
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
// is the actual item data thats stored
pub trait ItemInstanceData: Sized {
    type ItemNode: GodotClass + ItemInstance + Inherits<Node3D> + Bounds<Declarer = DeclUser>;
    const ITEM_ID: &str;

    // constructor for default data
    fn new() -> Self;

    // constructs from an item's data
    fn new_from_item(item: Gd<Self::ItemNode>) -> Self;

    // when item is equipped, load data into item
    fn load_into_item(&self, item: Gd<Self::ItemNode>);

    // when item is unequipped, take data from item
    fn take_from_item(&mut self, item: Gd<Self::ItemNode>);

    // idk
    // fn try_id() -> Result<StringName, ()> {
    //     StringName::from_str(Self::ITEM_ID).map_err(|_| ())
    // }
}

// wrapper for the ItemInstanceData thats type erased
// so it can load into like a item library and be accessed
// via a identifier
pub trait ItemInstanceDataObject: Any {
    fn item_id(&self) -> &'static str;
    fn load_into(&self, node: Gd<Node3D>) -> Result<(), ItemLibraryErr>;
    fn take_from(&mut self, node: Gd<Node3D>) -> Result<(), ItemLibraryErr>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// blanket impls for every ItemInstanceData
impl<I: ItemInstanceData + 'static> ItemInstanceDataObject for I {
    fn item_id(&self) -> &'static str {
        I::ITEM_ID
    }

    fn load_into(&self, node: Gd<Node3D>) -> Result<(), ItemLibraryErr> {
        let node = node
            .try_cast::<I::ItemNode>()
            .map_err(|_| ItemLibraryErr::WrongNodeType)?;
        self.load_into_item(node);
        Ok(())
    }

    fn take_from(&mut self, node: Gd<Node3D>) -> Result<(), ItemLibraryErr> {
        let node = node
            .try_cast::<I::ItemNode>()
            .map_err(|_| ItemLibraryErr::WrongNodeType)?;
        self.take_from_item(node);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
