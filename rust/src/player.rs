use godot::classes::{IRigidBody3D, Input, RigidBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
struct Player {
    #[export]
    walk_speed: f32,
    #[export]
    run_speed: f32,
    #[export]
    response: f32,
    #[export]
    max_force: f32,
    base: Base<RigidBody3D>,
}

#[godot_api]
impl IRigidBody3D for Player {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            walk_speed: 2.0,
            run_speed: 3.0,
            response: 50.0,
            max_force: 100.0,
            base,
        }
    }

    fn physics_process(&mut self, _dt: f64) {
        let input = Input::singleton();

        let walk_speed = if input.is_action_pressed("sprint") {
            self.run_speed
        } else {
            self.walk_speed
        };
        let response = self.response;
        let max_force = self.max_force;

        let input_vec = input.get_vector("walk_left", "walk_right", "walk_forward", "walk_back");
        let target_velocity = Vector3 {
            x: input_vec.x,
            y: 0.0,
            z: input_vec.y,
        } * walk_speed as f32;

        let mut b = self.base_mut();
        let mass = b.get_mass();
        let velocity_error = target_velocity - b.get_linear_velocity();

        b.apply_central_force((velocity_error * response * mass).limit_length(Some(max_force)));
    }

    // fn unhandled_input(&mut self, event: Gd<InputEvent>) {}
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, _amount: f64) {
        self.signals().speed_increased().emit();
    }

    #[signal]
    fn speed_increased();
}
