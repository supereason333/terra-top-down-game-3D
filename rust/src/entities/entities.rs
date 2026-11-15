use godot::{
    classes::{IRigidBody3D, RayCast3D, RigidBody3D},
    prelude::*,
};

use crate::{
    other_stuff::health_node::HealthNode,
    player::{self, player_node::Player},
};

// TODO maybe impliment dyn entitty controller which exists outside of base entity?
#[derive(GodotClass)]
#[class(init, base=RigidBody3D)]
pub struct TestEntity {
    #[export]
    raycast: Option<Gd<RayCast3D>>,
    #[export]
    health_node: Option<Gd<HealthNode>>,

    #[export]
    speed: f32,
    #[export]
    response: f32,
    #[export]
    max_force: f32,

    player: Option<Gd<Player>>,
    last_seen_pos: Option<Vector3>,

    base: Base<RigidBody3D>,
}

#[godot_api]
impl IRigidBody3D for TestEntity {
    fn ready(&mut self) {}

    fn physics_process(&mut self, _dt: f64) {
        let target = self.update_target();
        let own_pos = self.base().get_global_position();

        let desired = match target {
            Some(t) if (t - own_pos).length_squared() > 0.25 => {
                (t - own_pos).normalized() * self.speed
            }
            Some(_) => {
                // arrived at last seen position
                self.last_seen_pos = None;
                Vector3::ZERO
            }
            None => Vector3::ZERO,
        };

        let mass = self.base().get_mass();
        let velocity_error = desired - self.base().get_linear_velocity();

        let force = (velocity_error * self.response * mass).limit_length(Some(self.max_force));
        let force = Vector3 {
            x: force.x,
            y: 0.0,
            z: force.z,
        };
        self.base_mut().apply_central_force(force);
    }
}

#[godot_api]
impl TestEntity {
    #[func]
    fn _on_body_entered_vision(&mut self, node: Gd<Node3D>) {
        match node.try_cast::<Player>() {
            Ok(player) => self.player = Some(player),
            Err(_e) => {}
        }
    }

    #[func]
    fn _on_body_exited_vision(&mut self, node: Gd<Node3D>) {
        match node.try_cast::<Player>() {
            Ok(_player) => self.player = None,
            Err(_e) => {}
        }
    }

    pub fn get_health(&self) -> Option<Gd<HealthNode>> {
        self.health_node.clone()
    }

    fn update_target(&mut self) -> Option<Vector3> {
        let player = self.player.clone()?;
        let mut ray = self.raycast.clone()?;
        let player_pos = player.get_global_position();

        let local = ray.to_local(player_pos);
        ray.set_target_position(local);
        ray.force_raycast_update();

        let sees_player = ray
            .get_collider()
            .is_some_and(|c| c.try_cast::<Player>().is_ok());

        if sees_player {
            self.last_seen_pos = Some(player_pos);
        }
        self.last_seen_pos
    }
}
