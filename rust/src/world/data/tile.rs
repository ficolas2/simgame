#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_id: u32,
    pub culled_by: Option<i32>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            tile_id: 0,
            culled_by: None,
        }
    }
}
