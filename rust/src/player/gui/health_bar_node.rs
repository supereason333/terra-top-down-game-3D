use godot::{
    classes::{IProgressBar, ProgressBar},
    prelude::*,
};

use crate::other_stuff::health_node::HealthNode;

#[derive(GodotClass)]
#[class(init, base=ProgressBar)]
struct HealthBar {
    #[export]
    health: Option<Gd<HealthNode>>,

    base: Base<ProgressBar>,
}

#[godot_api]
impl IProgressBar for HealthBar {
    fn ready(&mut self) {
        let health = if let Some(health) = self.health.clone() {
            health
        } else {
            return;
        };

        let mut b = self.base_mut();
        b.set_max(health.bind().max_health() as f64);
        b.set_value(health.bind().health() as f64);
        drop(b);
        health
            .signals()
            .tree_entered()
            .connect_other(&self.to_gd(), Self::on_health_ready);
    }
}

#[godot_api]
impl HealthBar {
    #[func]
    fn on_health_changed(&mut self, new: f32) {
        self.base_mut().set_value(new as f64);
    }

    #[func]
    fn on_health_ready(&mut self) {
        if let Some(health) = self.health.clone() {
            let mut b = self.base_mut();
            b.set_max(health.bind().max_health() as f64);
            b.set_value(health.bind().health() as f64);
        }
    }
}
