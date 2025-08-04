use godot::prelude::*;

struct WorldExtension;

#[gdextension]
unsafe impl ExtensionLibrary for WorldExtension {}

pub mod world {
    pub mod world;
    pub mod world_renderer;
    pub mod world_gen;
    pub mod data {
        pub mod chunk;
        pub mod tile;
    }
}

pub mod utils {
    pub mod tile_utils;
}
