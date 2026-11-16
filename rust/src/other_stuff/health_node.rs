use godot::{global::is_zero_approx, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct HealthNode {
    #[export]
    health: f32,
    #[export]
    max_health: f32,

    base: Base<Node>,
}

#[godot_api]
impl INode for HealthNode {
    fn ready(&mut self) {
        self.health = self.max_health;
    }
    fn process(&mut self, dt: f32) {
        self.update_health(0.5 * dt);
    }
}

#[godot_api]
impl HealthNode {
    pub fn update_health(&mut self, dmg: f32) {
        self.health += dmg;
        let health = self.health;
        if health < 0.0 || is_zero_approx(health as f64) {
            self.signals().death().emit();
        }
        self.health = self.health.clamp(0.0, self.max_health);
        let health = self.health;
        self.signals().health_changed().emit(health);
    }

    pub fn max_health(&self) -> f32 {
        self.max_health
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    #[signal]
    fn health_changed(new: f32);
    #[signal]
    fn death();
}
