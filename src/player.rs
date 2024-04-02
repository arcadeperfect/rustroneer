use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{Damping, ExternalForce, RigidBody},
    geometry::Collider,
};

use crate::{bevy_planet::lib::BevyPlanet, traits::IntoVec2, ui_state};

pub struct MyPlayerPlugin;

impl Plugin for MyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, apply_angle));
    }
}

#[derive(Component)]
pub struct MyPlayerTag;

fn spawn_player(mut cmd: Commands, ui_state: Res<ui_state::UiState>) {
    let spawn = get_spawn_point(ui_state);

    cmd.spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.5))
        .insert(Name::new("gravity player"))
        .insert(TransformBundle::from(Transform::from_xyz(
            spawn.x, spawn.y, 0.0,
        )))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(MyPlayerTag)
        .insert(Damping {
            linear_damping: 1.,
            angular_damping: 7.0,
        });
}

fn get_spawn_point(ui_state: Res<ui_state::UiState>) -> Vec3 {
    let mut y_pos = 0.0;

    // let dimension = p.get_dimension();
    let scale = ui_state.scale;
    let radius = ui_state.radius;
    y_pos = scale * radius;
    y_pos += y_pos * 0.1;

    Vec3::new(0.0, y_pos, 0.0)
}

fn move_player(
    mut query: Query<(&Transform, &mut ExternalForce), With<MyPlayerTag>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ui_state: Res<ui_state::UiState>,
) {
    let move_force = ui_state.player_move_force;
    let jetpack_force = ui_state.player_jetpack_force;

    for (t, mut ef) in query.iter_mut() {
        ef.force = Vec2::new(0.0, 0.0);

        let grav_scale = 5.;
        ef.force = -t.translation.into_vec2().normalize() * grav_scale;

        let direction = t.translation.truncate().normalize(); // Normalize to get direction
        let perp_clockwise = Vec2::new(-direction.y, direction.x); // Rotate 90 degrees clockwise
        let perp_counter_clockwise = Vec2::new(direction.y, -direction.x); // Rotate 90 degrees counter-clockwise

        if keyboard_input.pressed(KeyCode::KeyW) {
            ef.force += direction * jetpack_force;
            // println!("up\t {}", ef.force);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            ef.force -= direction * move_force;
            // println!("down\t {}", ef.force);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            ef.force += perp_clockwise * move_force;
            // println!("left\t {}", ef.force);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            ef.force += perp_counter_clockwise * move_force;
            // println!("right\t {}", ef.force);
        }

        // println!("{}", ef.force);
    }
}

fn apply_angle(mut q: Query<&mut Transform, With<MyPlayerTag>>) {
    let mut transform = q.single_mut();

    let distance_to_center = distance_squared(
        &Vec2::new(transform.translation.x, transform.translation.y),
        &Vec2::new(0.0, 0.0),
    );

    // let t = (distance_to_center / 25.).clamp(0., 1.);
    let t = (distance_to_center / 100.).clamp(0., 1.);

    let direction = transform.translation - Vec3::ZERO; // Vec3::ZERO is the origin
    let angle = f32::atan2(direction.x, direction.y);

    let target_rotatin = Quat::from_rotation_z(-angle);
    let current_rotation = transform.rotation;
    let new_rotation = current_rotation.slerp(target_rotatin, 0.01);

    println!("{}", t);

    transform.rotation = current_rotation.lerp(new_rotation, t);

    // transform.rotation = new_rotation;
}

// fn apply_angle(
//     mut query: Query<(&Transform, &mut ExternalForce), With<MyPlayerTag>>,
//     ui_state: Res<ui_state::UiState>,
// ) {
//     for (t, mut ef) in query.iter_mut() {
//         let direction = t.translation - Vec3::ZERO; // Vec3::ZERO is the origin
//         let angle_to_center = f32::atan2(direction.x, direction.y);
//         let local_down_angle = -t.rotation.to_axis_angle().1;
//         let delta = local_down_angle - angle_to_center;

//         println!("angle_to_center: {}", angle_to_center);
//         println!("local_down_angle: {}", local_down_angle);
//         println!("delta: {}", delta);

//         ef.torque = delta.clamp(-1., 1.) * ui_state.player_rotate_force;
//     }
// }

pub fn distance_squared(t1: &Vec2, t2: &Vec2) -> f32 {
    let x = t1.x - t2.x;
    let y = t1.y - t2.y;
    x * x + y * y
}
