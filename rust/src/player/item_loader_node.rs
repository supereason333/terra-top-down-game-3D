use godot::prelude::*;

use crate::GlobalSingleton;
use crate::inventory_system::item::{TestItem3D, TestItemInstanceData};
use crate::inventory_system::{ItemInstanceData, ItemInstanceDataObject};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct ItemHolder {
    #[export]
    item: StringName,

    node_ref: Option<Gd<Node3D>>,
    ins_data: Option<Box<dyn ItemInstanceDataObject>>,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for ItemHolder {
    fn ready(&mut self) {
        let mut item_lib = match GlobalSingleton::singleton().bind_mut().get_item_lib() {
            Some(lib) => lib,
            None => {
                godot_print!("ITEM LOADER NODE SINGLETON NO ITEM LIB");
                return;
            }
        };
        let mut lib = item_lib.bind_mut();

        let ins_data = match lib.get_item_data(&self.item) {
            Some(data) => data,
            None => return,
        };
        let node = match lib.get_item_node(ins_data.as_ref()) {
            Ok(node) => node,
            Err(_e) => {
                godot_print!("ITEM LOADER NODE CANOT GET ITEM NODE");
                return;
            }
        };

        self.base_mut().add_child(&node);

        self.ins_data = Some(ins_data);
        self.node_ref = Some(node);
    }
}
