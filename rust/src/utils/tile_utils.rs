use godot::builtin::Vector3;

pub const TILE_SIZE: f32 = 32.0;

pub const TILE_WIDTH: f32 = TILE_SIZE;
pub const TILE_HEIGHT: f32 = TILE_SIZE / 2.0;

pub fn iso_to_world(x: i32, y: i32, z: i32) -> Vector3 {
    let px = (x - y) as f32 * 0.5;
    let py = -(x + y - z) as f32 * 0.25;
    let order = (x + y) as f32 * 20.0 + z as f32;
    return Vector3::new(px, py, order);
}
