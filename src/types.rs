use bevy::ecs::system::Resource;
use planet::{types::Coord, PlanetOptions};

use crate::ui_state::UiState;



impl From<UiState> for PlanetOptions {
    fn from(ui_state: UiState) -> Self {
        Self {
            seed: 0,
            min_room_size: 20,

            // frequency: ui_state.frequency,
            // amplitude: ui_state.amplitude,
            radius: ui_state.radius,
            resolution: ui_state.resolution,
            thresh: ui_state.ca_thresh,
            // ca_iterations: ui_state.ca_iterations,
            weight: ui_state.ca_init_weight,
            blur: ui_state.blur,
            crust_thickness: ui_state.crust_thickness,
            ca_search_radius: ui_state.ca_searh_radius,
            ca_iterations: ui_state.ca_iterations,
            ca_misc: ui_state.ca_misc,
            invert_ca: ui_state.invert_ca,

            mask_frequency: ui_state.mask_frequency,
            mask_z: ui_state.mask_z,
            global_amplitude: ui_state.global_amplitude,
            // ..Default::default()
            displacement_scale: ui_state.displacement_scale,
            displacement_frequency: ui_state.displacement_frequency,
        }
    }
}





// fn dist_squared(a: &Coord, b: &Coord) -> f32 {
//     let dx = a.x as i32 - b.x as i32;
//     let dy = a.y as i32 - b.y as i32;
//     (dx * dx + dy * dy) as f32
// }

// fn dist(a: &Coord, b: &Coord) -> f32 {
//     let dx = a.x as i32 - b.x as i32;
//     let dy = a.y as i32 - b.y as i32;
//     ((dx * dx + dy * dy) as f32).sqrt()
// }
