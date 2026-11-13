use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct ItemHolder {
    #[export]
    item: StringName,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for ItemHolder {
    fn ready(&mut self) {}
}
