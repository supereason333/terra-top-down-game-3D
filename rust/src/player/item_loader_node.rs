use godot::classes::InputEvent;
use godot::prelude::*;

use crate::GlobalSingleton;
use crate::inventory_system::ItemInstanceDataObject;

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

        let mut ins_data = match lib.get_item_data(&self.item) {
            Some(data) => data,
            None => return,
        };
        let (node, _resource) = match lib.get_item_node(ins_data.as_mut()) {
            Ok(node) => node,
            Err(e) => {
                godot_print!("Item loader cannot get item node {:?}", e);
                return;
            }
        };

        self.base_mut().add_child(&node);

        self.ins_data = Some(ins_data);
        self.node_ref = Some(node);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Some(item) = &self.ins_data {
            item.as_ref().pass_input(event);
        }
    }
}
