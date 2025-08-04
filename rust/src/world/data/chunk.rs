use super::tile::Tile;

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    tiles: [[[Tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            // initializes all elements to 0
            tiles: [[[Tile::new(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, z: usize, tile_id: u32) {
        self.tiles[z][y][x].tile_id = tile_id;
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Tile {
        self.tiles[z][y][x]
    }
}
