use godot::classes::{Camera3D, ICamera3D, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera3D)]
struct FollowCamera {
    #[export]
    target: Option<Gd<Node3D>>,
    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for FollowCamera {
    fn init(base: Base<Camera3D>) -> Self {
        Self {
            target: None,
            base: base,
        }
    }

    fn process(&mut self, _dt: f64) {
        let target_pos: Vector3;
        if let Some(target) = &mut self.target {
            target_pos = target.get_global_position();
        } else {
            return;
        }
        let mut b = self.base_mut();
        let y = b.get_global_position().y;
        b.set_global_position(Vector3 {
            x: target_pos.x,
            y: y,
            z: target_pos.z,
        });
    }
}
