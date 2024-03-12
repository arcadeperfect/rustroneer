use bevy::ecs::system::Resource;
use planet::PlanetOptions;

#[derive(Resource, Debug, Clone)]
pub struct UiState {
    pub changed: bool,
    pub frequency: f32,
    pub amplitude: f32,
    pub radius: f32,
    pub resolution: u32,
    pub thresh: u32,
    pub iterations: u32,
    pub weight: f32,
    pub blur: f32,
    pub selected_option: String,
    pub scale: f32,
    pub show_texture: bool,
    pub show_vectors: bool,
    pub show_debug: bool
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            changed: false,
            frequency: 0.,
            amplitude: 0.,
            radius: 1.,
            resolution: 100,
            thresh: 4,
            iterations: 10,
            weight: 0.62,
            blur: 1.,
            scale: 5.,
            show_texture: true,
            show_vectors: true,
            show_debug: true,

            selected_option: "planet".to_string(),
        }
    }
}

impl From<UiState> for PlanetOptions {
    fn from(ui_state: UiState) -> Self {
        Self {
            seed: 0,
            min_room_size: 20,

            frequency: ui_state.frequency,
            amplitude: ui_state.amplitude,
            radius: ui_state.radius,
            resolution: ui_state.resolution,
            thresh: ui_state.thresh,
            iterations: ui_state.iterations,
            weight: ui_state.weight,
            blur: ui_state.blur,
        }
    }
}