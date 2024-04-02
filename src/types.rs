
use planet::PlanetOptions;

use crate::ui_state::UiState;

impl From<UiState> for PlanetOptions {
    fn from(ui_state: UiState) -> Self {
        Self {
            seed: 0,
            min_room_size: 20,
            radius: ui_state.radius,
            resolution: ui_state.resolution,
            ca_options: ui_state.ca_options,
            global_noise_options: ui_state.global_noise_options,
            noise_mask_options: ui_state.noise_mask_options,



            // ca_thresh: ui_state.ca_thresh,
            // ca_weight: ui_state.ca_init_weight,
            // ca_search_radius: ui_state.ca_searh_radius,
            // ca_iterations: ui_state.ca_iterations,
            // ca_misc: ui_state.ca_misc,
            // invert_ca: ui_state.invert_ca,

            blur: ui_state.blur,
            crust_thickness: ui_state.crust_thickness,
            // mask_frequency: ui_state.mask_frequency,
            // mask_z: ui_state.mask_z,
            // global_amplitude: ui_state.global_amplitude,
            displacement_scale: ui_state.displacement_scale,
            displacement_frequency: ui_state.displacement_frequency,
            rooms: ui_state.rooms
        }
    }
}
