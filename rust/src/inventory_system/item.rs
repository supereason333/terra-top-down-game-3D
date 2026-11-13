use godot::prelude::*;

use crate::inventory_system::ItemInstanceData;

pub trait ItemInstance {}

#[derive(GodotClass)]
#[class(base=Node3D)]
struct TestItem3D {
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
}

struct TestItemInstanceData {
    counter: i32,
}

impl ItemInstanceData for TestItemInstanceData {
    type ItemNode = TestItem3D;

    fn new() -> Self {
        Self { counter: 0 }
    }

    fn new_from_item(item: Gd<Self::ItemNode>) -> Option<Self> {
        None
    }

    fn load_into_item(&self, item: Gd<Self::ItemNode>) {
        // ite
    }

    fn take_from_item(&mut self, item: Gd<Self::ItemNode>) {}
}
