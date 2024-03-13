use bevy::prelude::*;
use bevy_rapier2d::{dynamics::{Damping, ExternalForce, RigidBody}, geometry::Collider};


pub struct MyPlayerPlugin;

impl Plugin for MyPlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player,apply_angle));
    }
}

#[derive(Component)]
pub struct PlayerTag;

fn spawn_player(mut cmd: Commands) {
    cmd.spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.5))
        .insert(Name::new("gravity player"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 8.0, 0.0)))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(PlayerTag)
        .insert(Damping { linear_damping: 1., angular_damping: 1.0 })
        ;
}

fn move_player(mut query: Query<(&Transform, &mut ExternalForce), With<PlayerTag>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let move_force = 12.0;
    let jetpack_force = 12.0;

    for (transform, mut ef) in query.iter_mut() {
       
        let direction = transform.translation.truncate().normalize(); // Normalize to get direction
        let perp_clockwise = Vec2::new(-direction.y, direction.x); // Rotate 90 degrees clockwise
        let perp_counter_clockwise = Vec2::new(direction.y, -direction.x); // Rotate 90 degrees counter-clockwise

        if keyboard_input.pressed(KeyCode::KeyW) {
            ef.force += direction * jetpack_force;
            println!("move");
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            ef.force -= direction * move_force;
            println!("move");
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            ef.force += perp_clockwise * move_force;
            println!("move");
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            ef.force += perp_counter_clockwise * move_force;
            println!("move");
        }
    }

}

fn apply_angle(mut q: Query <&mut Transform, With<PlayerTag>>){

    let mut transform = q.single_mut();
    let direction = transform.translation - Vec3::ZERO; // Vec3::ZERO is the origin
    let angle = f32::atan2(direction.x, direction.y);

    transform.rotation = Quat::from_rotation_z(-angle);

}