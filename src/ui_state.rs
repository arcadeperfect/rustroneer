use std::env;
use std::fs;

use anyhow::Result;
use bevy::ecs::system::Resource;
use planet::types::CaOptions;
use planet::types::FractalNoiseOptions;
use planet::types::GlobalNoiseOptions;
use planet::types::NoiseMaskOptions;
use serde::{Deserialize, Serialize};
use serde_yaml;
use strum_macros::EnumIter;

use crate::planet_gizmos::GizmoOptions;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum BitmapDisplay {
    PlanetRaw,
    PlanetProcessed,
    Altitude,
    Depth,
    Mask,
    RoomsRaw,
    RoomsDebug,
    TileMapDebug,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct UiState {
    pub changed: bool,
    pub fractal_noises: Vec<FractalNoiseOptions>,
    pub noise_mask_options: NoiseMaskOptions,
    pub global_noise_options: GlobalNoiseOptions,
    pub radius: f32,
    pub resolution: u32,
    pub ca_options: CaOptions,
    pub blur: f32,
    pub bitmap_dislpay: BitmapDisplay,
    pub scale: f32,
    pub show_texture: bool,
    pub show_vectors: bool,
    pub show_debug: bool,
    pub crust_thickness: f32,
    pub displacement_scale: f64,
    pub displacement_frequency: f64,
    pub invert_ca: bool,
    pub gizmo_options: GizmoOptions,
    pub rooms: bool,
    pub player_jetpack_force: f32,
    pub player_move_force: f32,
    pub player_rotate_force: f32,
    pub game_camera: bool,
    pub game_camera_zoom: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            changed: false,
            fractal_noises: vec![
                FractalNoiseOptions::default(),
                FractalNoiseOptions::default(),
                FractalNoiseOptions::default(),
            ],
            noise_mask_options: NoiseMaskOptions::default(),
            global_noise_options: GlobalNoiseOptions::default(),
            radius: 1.,
            resolution: 200,
            ca_options: CaOptions::default(),
            blur: 1.,
            scale: 100.,
            show_texture: true,
            show_vectors: true,
            show_debug: false,
            crust_thickness: 0.0,
            bitmap_dislpay: BitmapDisplay::PlanetRaw,
            displacement_scale: 0.0,
            displacement_frequency: 0.0,
            invert_ca: false,
            gizmo_options: GizmoOptions::default(),
            rooms: false,
            player_jetpack_force: 18.0,
            player_move_force: 18.0,
            player_rotate_force: 1.0,
            game_camera: false,
            game_camera_zoom: 30.0,
        }
    }
}

impl UiState {
    pub fn save(&self) -> Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        let file_path = env::current_dir()?.join("save/save.yaml");
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, yaml)?;

        Ok(())
    }

    pub fn load() -> Result<UiState> {
        let file_path = env::current_dir()?.join("save/save.yaml");
        let contents = fs::read_to_string(file_path)?;
        let v = serde_yaml::from_str(&contents)?;
        Ok(v)
    }
}
