use godot::prelude::*;

use super::world::World;

#[derive(GodotClass)]
#[class(base=Node)]
struct WorldRenderer {
    #[export]
    world: Option<Gd<World>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for WorldRenderer {
    fn init(base: Base<Node>) -> Self {
        Self { world: None, base }
    }
}
