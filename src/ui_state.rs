use std::env;
use std::fs;

use anyhow::Result;
use bevy::ecs::system::Resource;
use planet::types::FractalNoiseOptions;
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
    pub noise: Vec<FractalNoiseOptions>,
    pub mask_frequency: f64,
    pub mask_z: f64,
    pub global_amplitude: f32,
    pub radius: f32,
    pub resolution: u32,
    pub ca_thresh: u32,
    pub ca_iterations: u32,
    pub ca_init_weight: f32,
    pub ca_searh_radius: u32,
    pub ca_misc: i32,
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
    pub rooms: bool
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            changed: false,
            noise: vec![
                FractalNoiseOptions::default(),
                FractalNoiseOptions::default(),
                FractalNoiseOptions::default(),
            ],

            global_amplitude: 1.0,
            mask_frequency: 0.5,
            mask_z: 0.0,
            radius: 1.,
            resolution: 200,
            ca_thresh: 4,
            ca_iterations: 1,
            ca_init_weight: 0.62,
            ca_misc: 0,
            blur: 1.,
            scale: 100.,
            show_texture: true,
            show_vectors: true,
            show_debug: false,
            crust_thickness: 0.0,
            ca_searh_radius: 3,
            bitmap_dislpay: BitmapDisplay::PlanetRaw,
            displacement_scale: 0.0,
            displacement_frequency: 0.0,
            invert_ca: false,
            gizmo_options: GizmoOptions::default(),
            rooms: false
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
