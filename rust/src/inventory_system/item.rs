use std::str::FromStr;

use godot::{
    classes::{InputEvent, InputEventMouseButton},
    prelude::*,
};

use crate::{GlobalSingleton, inventory_system::ItemInstanceData};

pub trait ItemInstance {}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct TestItem3D {
    counter: i32,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for TestItem3D {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            counter: 1,
            base: base,
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.is_pressed() {
                godot_print!("Pressed");
            }
        }
    }
}

impl ItemInstance for TestItem3D {}

impl TestItem3D {
    fn up_counter(&mut self) {
        self.counter += 1;
    }
}

pub struct TestItemInstanceData {
    counter: i32,
}

impl ItemInstanceData for TestItemInstanceData {
    type ItemNode = TestItem3D;
    const ITEM_ID: &str = "test_item";

    fn new() -> Self {
        Self { counter: 0 }
    }

    fn new_from_item(item: Gd<Self::ItemNode>) -> Self {
        let ins = item.bind();
        Self {
            counter: ins.counter,
        }
    }

    fn load_into_item(&self, mut item: Gd<Self::ItemNode>) {
        let mut ins = item.bind_mut();
        ins.counter = self.counter;
    }

    fn take_from_item(&mut self, item: Gd<Self::ItemNode>) {
        let ins = item.bind();
        self.counter = ins.counter;
    }
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct BasicGun3D {
    counter: i32,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for BasicGun3D {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            counter: 1,
            base: base,
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.is_pressed() {
                let counter = self.counter;
                // godot_print!("{counter}");

                self.counter += 1;

                self.fire();
            }
        }
    }

    fn ready(&mut self) {
        godot_print!("Gun ready");
    }
}

impl ItemInstance for BasicGun3D {}

impl BasicGun3D {
    fn fire(&mut self) {
        let mut global = GlobalSingleton::singleton();
        let mut proj_node = match global.bind_mut().get_projectiles_node() {
            Some(node) => node,
            None => return,
        };
        let proj_lib = match global.bind_mut().get_projectile_lib() {
            Some(lib) => lib,
            None => return,
        };
        let (node, res) = match proj_lib
            .bind()
            .get_projectile_node(&StringName::from_str("test_projectile").unwrap())
        {
            Ok(n) => n,
            Err(_e) => return,
        };

        proj_node.bind_mut().register_projectile(
            node.clone(),
            res.bind()._get_lifetime(),
            self.base().get_global_transform(),
        );
    }
}

pub struct BasicGunInstanceData {
    counter: i32,
}

impl ItemInstanceData for BasicGunInstanceData {
    type ItemNode = BasicGun3D;
    const ITEM_ID: &str = "basic_gun";

    fn new() -> Self {
        Self { counter: 0 }
    }

    fn new_from_item(item: Gd<Self::ItemNode>) -> Self {
        let ins = item.bind();
        Self {
            counter: ins.counter,
        }
    }

    fn load_into_item(&self, mut item: Gd<Self::ItemNode>) {
        let mut ins = item.bind_mut();
        ins.counter = self.counter;
    }

    fn take_from_item(&mut self, item: Gd<Self::ItemNode>) {
        let ins = item.bind();
        self.counter = ins.counter;
    }
}
