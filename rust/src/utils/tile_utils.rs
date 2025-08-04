use godot::builtin::Vector2;

pub const TILE_SIZE: f32 = 32.0;

pub const TILE_WIDTH: f32 = TILE_SIZE;
pub const TILE_HEIGHT: f32 = TILE_SIZE / 2.0;

pub fn iso_to_screen(x: i32, y: i32, z: i32) -> Vector2 {
    let px = (x - y) as f32 * TILE_WIDTH * 0.5;
    let py = (x + y - z) as f32 * TILE_HEIGHT * 0.5;
    return Vector2::new(px, py);
}
