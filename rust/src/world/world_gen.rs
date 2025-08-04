use noise::{NoiseFn, Perlin};
use rand::Rng;

use super::data::chunk::{CHUNK_SIZE, Chunk};

pub fn generate_chunk(chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Chunk {
    let perlin = Perlin::new(42);
    let mut rng = rand::rng();
    let frequency = 0.1;

    let mut chunk = Chunk::new();
    for x_shift in 0..CHUNK_SIZE {
        let x = x_shift as i32 + chunk_x * 32;
        for y_shift in 0..CHUNK_SIZE {
            let tile_rng = rng.random_range(0..3);
            let y = y_shift as i32 + chunk_y * 32;

            if chunk_z == 0 {
                let n = perlin.get([x as f64 * frequency, y as f64 * frequency]);

                let h = (((n + 1.0) * 0.5) * 5.0) as i32 + 1;

                for z in 0..h {
                    if z == h - 1 && z >= 1 {
                        chunk.set_tile(x_shift, y_shift, z as usize, 23);
                    } else {
                        chunk.set_tile(x_shift, y_shift, z as usize, 1 + tile_rng);
                    }
                }
                if h < 2 {
                    chunk.set_tile(x_shift, y_shift, 1, 111);
                    chunk.set_tile(x_shift, y_shift, 0, 4);
                }
            } else if chunk_z < 0 {
                for z in 0..CHUNK_SIZE {
                    chunk.set_tile(x_shift, y_shift, z, 64);
                }
            }
        }
    }
    chunk
}
