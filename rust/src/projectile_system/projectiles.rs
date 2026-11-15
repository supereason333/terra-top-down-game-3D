use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct BasicProjectile3D {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for BasicProjectile3D {}
