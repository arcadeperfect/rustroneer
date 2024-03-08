use bevy::{prelude::*, render::camera::ScalingMode};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
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