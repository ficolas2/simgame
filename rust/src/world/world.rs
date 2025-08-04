use std::collections::HashMap;

use godot::prelude::*;

use super::data::chunk::Chunk;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,

    chunks: HashMap<(i32, i32, i32), Chunk>,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        let mut world = Self {
            base,
            chunks: HashMap::new(),
        };

        let mut chunk = Chunk::new();
        for x in 0..32 {
            for y in 0..32 {
                chunk.set_tile(x, y, 0, 1);
            }
        }
        world.chunks.insert((0,0,0), chunk);

        world
    }
}

impl World {
    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> &Chunk {
        return self.chunks.get(&(x, y, z)).unwrap()
    }
}
