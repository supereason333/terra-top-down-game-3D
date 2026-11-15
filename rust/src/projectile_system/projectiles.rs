use godot::{classes::Area3D, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct TestProjectile3D {
    #[export]
    detect_area: Option<Gd<Area3D>>,
    #[export]
    velocity: f32,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for TestProjectile3D {
    fn ready(&mut self) {}

    fn physics_process(&mut self, dt: f64) {
        let b = self.base();
        let pos = b.get_position();
        let forward = -b.get_transform().basis.col_c();
        let pos = pos + forward * self.velocity * (dt as f32);
        let mut b = self.base_mut();
        b.set_position(pos);
    }
}

#[godot_api]
impl TestProjectile3D {
    #[func]
    fn _on_body_entered(&mut self, _body: Gd<Node3D>) {
        self.base_mut().queue_free();
    }
}
