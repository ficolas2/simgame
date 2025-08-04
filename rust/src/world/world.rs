use std::collections::HashMap;

use godot::prelude::*;

use super::{data::{chunk::{Chunk, CHUNK_SIZE}, tile::Tile}, world_gen};

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

        for x in -1..1 {
            for y in -1..1 {
                for z in -1..1 {
                    let chunk = world_gen::generate_chunk(x, y, z);
                    world.chunks.insert((x, y, z), chunk);
                }
            }
        }

        world
    }
}

impl World {
    pub fn get_chunk(&mut self, x: i32, y: i32, z: i32) -> &Chunk {
        return self.chunks.entry((x, y, z)).or_insert_with(|| Chunk::new());
    }

    pub fn get_tile(&mut self, x: i32, y: i32, z: i32) -> Tile {
        let chunk_size = CHUNK_SIZE as i32;
        let chunk = self.get_chunk(
            x.div_euclid(chunk_size),
            y.div_euclid(chunk_size),
            z.div_euclid(chunk_size),
        );
        let rel_x = x.rem_euclid(chunk_size) as usize;
        let rel_y = y.rem_euclid(chunk_size) as usize;
        let rel_z = z.rem_euclid(chunk_size) as usize;

        chunk.get_tile(rel_x, rel_y, rel_z)
    }
}
