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
}

impl ItemInstance for TestItem3D {}

impl TestItem3D {
    fn up_counter(&mut self) {
        self.counter += 1;
    }

    fn recieve_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.is_pressed() {
                godot_print!("Pressed");
            }
        }
    }
}

pub struct TestItemInstanceData {
    counter: i32,
    item: Option<Gd<TestItem3D>>,
}

impl ItemInstanceData for TestItemInstanceData {
    type ItemNode = TestItem3D;
    const ITEM_ID: &str = "test_item";

    fn new() -> Self {
        Self {
            counter: 0,
            item: None,
        }
    }

    fn new_from_item(item: Gd<Self::ItemNode>) -> Self {
        let i = item.clone();
        let ins = item.bind();
        Self {
            counter: ins.counter,
            item: Some(i),
        }
    }

    fn load_into_item(&mut self, mut item: Gd<Self::ItemNode>) {
        let mut ins = item.bind_mut();
        ins.counter = self.counter;
    }

    fn take_from_item(&mut self, item: Gd<Self::ItemNode>) {
        let ins = item.bind();
        self.counter = ins.counter;
    }

    fn pass_input_to(&self, event: Gd<InputEvent>) {}
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

    fn ready(&mut self) {
        godot_print!("Gun ready");
    }
}

impl ItemInstance for BasicGun3D {}

impl BasicGun3D {
    pub fn recieve_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.is_pressed() {
                self.counter += 1;

                self.fire();
            }
        }
    }

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
    item: Option<Gd<BasicGun3D>>,
}

impl ItemInstanceData for BasicGunInstanceData {
    type ItemNode = BasicGun3D;
    const ITEM_ID: &str = "basic_gun";

    fn new() -> Self {
        Self {
            counter: 0,
            item: None,
        }
    }

    fn new_from_item(item: Gd<Self::ItemNode>) -> Self {
        let i = item.clone();
        let ins = item.bind();
        Self {
            counter: ins.counter,
            item: Some(i),
        }
    }

    fn load_into_item(&mut self, mut item: Gd<Self::ItemNode>) {
        self.item = Some(item.clone());
        let mut ins = item.bind_mut();
        ins.counter = self.counter;
    }

    fn take_from_item(&mut self, item: Gd<Self::ItemNode>) {
        self.item = None;
        let ins = item.bind();
        self.counter = ins.counter;
    }

    fn pass_input_to(&self, event: Gd<InputEvent>) {
        if let Some(mut item) = self.item.clone() {
            item.bind_mut().recieve_input(event);
        }
    }
}
