use crate::{player::MyPlayerTag, ui::OccupiedScreenSpace, ui_state};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::camera::Viewport,
    window::PrimaryWindow,
};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, track_camera);
        app.add_systems(Update, set_camera_viewport);
        app.add_systems(Update, apply_angle);
        // app.add_plugins(TweeningPlugin);
    }
}

fn track_camera(
    mut query: Query<(&mut Transform, Option<&MyPlayerTag>), Or<(With<Camera>, With<MyPlayerTag>)>>,
    ui_state: Res<ui_state::UiState>,
) {
    let (mut camera_transform, player_transform) =
        query
            .iter_mut()
            .fold((None, None), |(camera, player), (transform, player_tag)| {
                if player_tag.is_some() {
                    (camera, Some(transform))
                } else {
                    (Some(transform), player)
                }
            });

    if let (Some(mut camera), Some(player)) = (camera_transform, player_transform) {
        match ui_state.game_camera {
            true => {
                let player_x = player.translation.x;
                let player_y = player.translation.y;
                let camera_x = camera.translation.x;
                let camera_y = camera.translation.y;
                let camera_z = camera.translation.z;
                let new_x = camera_x.lerp(player_x, 0.05);
                let new_y = camera_y.lerp(player_y, 0.05);
                let new_z = camera_z.lerp(ui_state.game_camera_zoom, 0.05);
                camera.translation.x = new_x;
                camera.translation.y = new_y;
                camera.translation.z = new_z;
            }

            false => {
                let current_translation = camera.translation;
                let target_translation = Vec3::new(-86., -0., 350.0);
                let new_translation = current_translation.lerp(target_translation, 0.05);

                camera.translation = new_translation;
            }
        }
    }
}

fn apply_angle(mut q: Query<&mut Transform, With<Camera>>, ui_state: Res<ui_state::UiState>) {
    let mut transform = q.single_mut();

    match ui_state.game_camera {
        true => {
            ({

                let distance_to_center = distance_squared(
                    &Vec2::new(transform.translation.x, transform.translation.y),
                    &Vec2::new(0.0, 0.0),
                );
                let t = (distance_to_center / 100.).clamp(0., 1.);


                // let mut transform = q.single_mut();
                let direction = transform.translation - Vec3::ZERO; // Vec3::ZERO is the origin
                let angle = f32::atan2(direction.x, direction.y);
                let target_rotatin = Quat::from_rotation_z(-angle);
                let current_rotation = transform.rotation;
                let new_rotation = current_rotation.slerp(target_rotatin, 0.01);


                transform.rotation = current_rotation.lerp(new_rotation, t);

                // transform.rotation = new_rotation;
            })
        }

        false => {
            let current_rotation = transform.rotation;
            let target_rotation = Quat::from_rotation_z(0.0);
            let new_rotation = current_rotation.slerp(target_rotation, 0.01);
            transform.rotation = new_rotation;
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-86., -0., 30.0),
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // commands.spawn(Camera3dBundle {

    //     transform: Transform::from_xyz(-86., -0., 30.0),

    //     ..Default::default()
    // });
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

pub fn distance_squared(t1: &Vec2, t2: &Vec2) -> f32 {
    let x = t1.x - t2.x;
    let y = t1.y - t2.y;
    x * x + y * y
}