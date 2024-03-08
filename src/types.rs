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
            scale: 3.,

            selected_option: "planet".to_string(),
        }
    }
}

