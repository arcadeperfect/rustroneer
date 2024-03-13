use bevy::ecs::system::Resource;
use planet::PlanetOptions;

use crate::ui::UiState;

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

            // ..Default::default()
        }
    }
}