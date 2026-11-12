mod item;

use godot::prelude::*;

#[derive(Debug)]
pub enum ItemLibraryErr {
    InitilizeError,
}

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct ItemLibrary {
    #[export]
    lib: Array<Gd<ItemDefinitionResource>>,
    initilized: bool,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemLibrary {
    fn init(base: Base<Resource>) -> Self {
        Self {
            lib: Array::new(),
            initilized: false,

            base: base,
        }
    }
}

#[godot_api]
pub impl ItemLibrary {
    /// initilises the library, checks for errors and such
    pub fn initilize(&mut self) -> Result<(), ItemLibraryErr> {
        self.initilized = true;
        Ok(())
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
    item_scene: Option<Gd<PackedScene>>,
    #[export]
    default_instance_resource: Option<Gd<ItemInstanceResource>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ItemDefinitionResource {
    //
}

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
