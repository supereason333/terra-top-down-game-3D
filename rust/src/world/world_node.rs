use godot::{classes::Timer, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct WorldNode {
    #[export]
    projectiles_node: Option<Gd<ProjectilesNode>>,

    base: Base<Node3D>,
}

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct ProjectilesNode {
    base: Base<Node3D>,
}

impl ProjectilesNode {
    pub fn register_projectile(&mut self, mut projectile: Gd<Node3D>, lifetime: f32) {
        let mut b = self.base_mut();
        let mut timer = Timer::new_alloc();
        timer.set_wait_time(lifetime as f64);
        timer.set_name("_DeathTimer");
        timer.set_autostart(true);
        timer.set_one_shot(true);
        timer
            .signals()
            .timeout()
            .connect_other(&projectile.clone().upcast::<Node>(), Node::queue_free);
        projectile.add_child(&timer);
        b.add_child(&projectile);
    }

    pub fn clear_projectiles(&mut self) {
        let b = self.base_mut();
        for mut child in b.get_children().iter_shared() {
            child.queue_free();
        }
    }
}
