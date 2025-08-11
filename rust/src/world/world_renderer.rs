use godot::{
    classes::{IMultiMeshInstance3D, MultiMeshInstance3D},
    prelude::*,
};

use crate::utils::tile_utils::iso_to_world;

use super::world::World;

const MAX_COUNT: i32 = 50000;

#[derive(GodotClass)]
#[class(base=MultiMeshInstance3D)]
struct WorldRenderer {
    #[export]
    world: Option<Gd<World>>,

    base: Base<MultiMeshInstance3D>,

    #[export]
    height: i32,
    #[export]
    z_count: i32,
}

#[godot_api]
impl IMultiMeshInstance3D for WorldRenderer {
    fn init(base: Base<MultiMeshInstance3D>) -> Self {
        Self {
            world: None,
            base,
            height: 10,
            z_count: 10,
        }
    }

    fn ready(&mut self) {
        let owner = self.base_mut();
        let mut mm = owner.get_multimesh().unwrap();
        mm.set_instance_count(MAX_COUNT);
    }

    fn process(&mut self, _delta: f64) {
        let height = self.height;
        let z_count = self.z_count;
        let mut world = self.world.clone();
        let owner = self.base_mut();

        let mut world = world.as_mut().unwrap().bind_mut();
        let mut mm = owner.get_multimesh().unwrap();

        let x_count = 50; // TODO hardcoded
        let y_count = 50;

        let mut transform_array = PackedVector3Array::new();
        let mut custom_data_array = VariantArray::new();

        let x_axis = Vector3::new(1.0, 0.0, 0.0);
        let y_axis = Vector3::new(0.0, 1.0, 0.0);
        let z_axis = Vector3::new(0.0, 0.0, 1.0);
        let zero = Vector3::new(0.0, 0.0, 0.0);
        // TODO: a few can be removed by only rendering the top ones from
        let zero_color = Color::from_rgba(0.0, 0.0, 0.0, 0.0);
        // the furthest y


        let mut i = 0;
        for row in 0..y_count + z_count {
            let start_x = (row + 1) / 2 - x_count;
            let start_y = row / 2 + 1;

            for column in 0..x_count {
                for z in (height-z_count)..=height {
                // for z in (height - z_count)..=height {
                    let x = start_x + column;
                    let y = start_y - column;

                    let tile = world.get_tile(x, y, z);
                    if tile.tile_id == 0 {
                        continue;
                    }
                    i+=1;

                    // Transform
                    let origin = iso_to_world(x, y, z);
                    // mm.set_instance_transform_2d(i, xf);
                    transform_array.push(x_axis);
                    transform_array.push(y_axis);
                    transform_array.push(z_axis);
                    transform_array.push(origin);

                    // UVs
                    let u = ((tile.tile_id - 1) % 11) as f32;
                    let v = ((tile.tile_id - 1) / 11) as f32;

                    let custom_data = Color::from_rgba(u/11.0, v/11.0, 0.0, 1.0);
                    custom_data_array.push(&custom_data.to_variant());
                }
            }
        }

        let visible_count = (transform_array.len() / 4) as i32;
        let max_count = MAX_COUNT;
        for _ in visible_count..max_count {
            transform_array.push(x_axis);
            transform_array.push(y_axis);
            transform_array.push(z_axis);
            transform_array.push(zero);

            custom_data_array.push(&zero_color.to_variant());
        }

        mm.set_visible_instance_count(visible_count);
        mm.set("transform_array", &Variant::from(transform_array));
        mm.set("custom_data_array", &Variant::from(custom_data_array));
    }
}

// fn get_custom_data(u: u8, v: u8, visible: bool) -> Vector4 {
//     let mut bits: u32 = 0;
//     bits |= u as u32;
//     bits |= (v as u32) << 8;
//     bits |= (visible as u32) << 16;
//     let r = f32::from_bits(bits);
//
//     Vector4::new(r, 1.0, 1.0, 1.0)
// }
