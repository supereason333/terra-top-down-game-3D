use godot::prelude::*;

use crate::inventory_system::ItemInstanceResource;

pub trait ItemInstance {
    fn set_instance(&mut self, instance: Gd<ItemInstanceResource>);
}

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct TestItem3D {
    instance: Option<Gd<ItemInstanceResource>>,
}

impl ItemInstance for TestItem3D {
    fn set_instance(&mut self, instance: Gd<ItemInstanceResource>) {
        self.instance = Some(instance);
    }
}
