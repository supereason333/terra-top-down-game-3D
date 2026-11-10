use godot::classes::{Control, IControl, InputEvent, InputEventMouseMotion};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
struct MouseTracker {
    base: Base<Control>,
}

#[godot_api]
impl IControl for MouseTracker {
    fn gui_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(evnt) = event.try_cast::<InputEventMouseMotion>() {
            let b = self.base();
            let pos = evnt.get_position() - b.get_size() / 2.0;
            self.signals().position_changed().emit(pos);
            // godot_print!("{pos}");
        }
    }
}

#[godot_api]
impl MouseTracker {
    #[signal]
    fn position_changed(pos: Vector2);
}
