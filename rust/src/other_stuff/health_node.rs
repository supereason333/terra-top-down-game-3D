use godot::prelude::*;

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
}

#[godot_api]
impl HealthNode {
    fn update_health(&mut self, dmg: f32) {
        self.health += dmg;
        if self.health < 0.0 {
            self.signals().death().emit();
        }
        self.health = self.health.clamp(0.0, self.max_health);
        let health = self.health;
        self.signals().health_changed().emit(health);
    }

    #[signal]
    fn health_changed(new: f32);
    #[signal]
    fn death();
}
