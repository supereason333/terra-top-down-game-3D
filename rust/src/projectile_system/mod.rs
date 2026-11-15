mod projectiles;

use std::collections::HashMap;

use godot::prelude::*;

#[derive(Debug)]
pub enum ProjectileLibraryErr {
    NoSuchProjectile,
    NoScene,
}

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct ProjectileLibrary {
    #[export]
    lib: Array<Gd<ProjectileDefinitionResource>>,
    initialized: bool,

    lookup: HashMap<StringName, Gd<ProjectileDefinitionResource>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for ProjectileLibrary {
    fn init(base: Base<Resource>) -> Self {
        Self {
            lib: Array::new(),
            initialized: false,

            lookup: HashMap::new(),

            base: base,
        }
    }
}

impl ProjectileLibrary {
    pub fn initialize(&mut self) -> Result<(), ProjectileLibraryErr> {
        for proj in self.lib.iter_shared() {
            let id = proj.bind().id.clone();
            if self.lookup.get(&id).is_some() {
                godot_print!("Two projectiles share the same id: {id}");
                continue;
            }
            self.lookup.insert(id, proj);
        }

        let registered = self.lookup.len();
        let total = self.lib.len();

        godot_print!("Registered {registered} projectiles out of {total}");

        self.initialized = true;
        Ok(())
    }

    pub fn get_projectile(&self, id: &StringName) -> Option<Gd<ProjectileDefinitionResource>> {
        match self.lookup.get(id) {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }

    pub fn get_projectile_node(
        &self,
        id: &StringName,
    ) -> Result<(Gd<Node3D>, Gd<ProjectileDefinitionResource>), ProjectileLibraryErr> {
        let item = self
            .get_projectile(id)
            .ok_or(ProjectileLibraryErr::NoSuchProjectile)?;

        let scene = item
            .bind()
            .item_scene
            .clone()
            .ok_or(ProjectileLibraryErr::NoScene)?;

        let node = scene.instantiate_as::<Node3D>();

        Ok((node, item))
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct ProjectileDefinitionResource {
    #[export]
    lifetime: f32,
    #[export]
    display_name: GString,
    #[export]
    id: StringName,
    #[export]
    pub item_scene: Option<Gd<PackedScene>>,
    base: Base<Resource>,
}

impl ProjectileDefinitionResource {
    pub fn _get_lifetime(&self) -> f32 {
        self.lifetime
    }
}
