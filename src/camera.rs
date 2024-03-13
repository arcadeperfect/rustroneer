use std::time::Duration;

use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};

use crate::ui::OccupiedScreenSpace;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::*;

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, set_camera_viewport);
        // app.add_plugins(TweeningPlugin);
    }
}

fn spawn_camera(mut commands: Commands) {
  
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-86., -0., 350.0),
        ..Default::default()
    });
}

fn set_camera_viewport(
    mut camera: Query<&mut Camera>,
    windows: Query<&Window, With<PrimaryWindow>>,
    occupied_screen_space: Res<OccupiedScreenSpace>,
) {
    let l = occupied_screen_space.left;

    let mut this_camera = camera.single_mut();

    let pos_x = l * 2.;
    let pos_y = 0;
    let window = windows.single();
    let width = window.width();
    let height = window.height();
    let width = (width * 2.) - l * 2.;
    let height = height * 2.;

    let new_viewport = Viewport {
        physical_position: UVec2::new(pos_x as u32, pos_y as u32),
        physical_size: UVec2::new(width as u32, height as u32),
        ..Default::default()
    };

    this_camera.viewport = Some(new_viewport);
}

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn(Camera2dBundle {
//         projection: OrthographicProjection {
//             scaling_mode: ScalingMode::FixedVertical(10.0), // Adjust this value as needed
//             ..Default::default()
//         },
//         transform: Transform::from_xyz(0.0, 0.0, 10.0) // Position the camera above the X/Y plane
//             .looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });
// }
