use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_rapier2d::{
    dynamics::{Damping, ExternalForce, RigidBody},
    geometry::Collider,
};
use rand::Rng;

use crate::{
    line::{LineList, LineMaterial, LineStrip}, traits::IntoVec2, ui::{GeneralUpdateEvent, RegeneratePlanetEvent}, ui_state::{self, CameraMode}, vector_shapes::{RCircle, RRectangle}
};

pub struct MyPlayerPlugin;

impl Plugin for MyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RocketStatusResource>();
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (user_input, apply_angle));
        app.add_systems(
            PostStartup,
            spawn_player_mesh_system,
        );
        app.add_systems(PostStartup, spawn_rocket_ystem);
        app.add_systems(Update, set_player_direction);
        app.add_systems(Update, update_rocket_mesh_system);
        app.add_event::<PlayerEvent>();
        app.add_systems(Update, reset_player_system);
        app.add_systems(Update, keyboard_input);
        // app.add_systems(Update, reset_player_system);
    }
}

#[derive(Event, Debug)]
pub struct PlayerEvent {
    pub event_type: PlayerEventType,
    pub ui_state: ui_state::UiState,
}

#[derive(Debug)]
pub enum PlayerEventType {
    Respawn,
    RefreshPlayer,
    RefreshCam,
    None,
}
impl Default for PlayerEventType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component)]
pub struct MyPlayerTag;

#[derive(Component)]
pub struct PlayerMeshTag;
#[derive(Component)]
pub struct PlayerMeshesParentTag;
#[derive(Component)]
pub struct RocketParentTag;
#[derive(Component)]
pub struct RocketTag;

#[derive(Component)]
pub struct Direction {
    left_right: LeftRight,
}
enum LeftRight {
    Left,
    Right,
}

#[derive(Resource, Default)]
pub struct RocketStatusResource {
    pub status: RocketStatus,
}
enum RocketStatus {
    Firing,
    NotFiring,
}
impl Default for RocketStatus {
    fn default() -> Self {
        Self::NotFiring
    }
}

fn spawn_player(
    mut cmd: Commands,
    ui_state: Res<ui_state::UiState>,
) {
    let spawn =
        get_spawn_point(ui_state.scale, ui_state.radius);

    // let spawn = Vec3::new(0.0, 0.0, 0.0);

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
        })
        .insert(Direction {
            left_right: LeftRight::Left,
        });
}

fn get_spawn_point(scale: f32, radius: f32) -> Vec3 {
    let mut y_pos: f32;

    // let dimension = p.get_dimension();
    // let scale = ui_state.scale;
    // let radius = ui_state.radius;
    y_pos = scale * radius;
    y_pos += y_pos * 0.1;

    Vec3::new(0.0, y_pos, 0.0)
}

fn spawn_player_mesh_system(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    // mut mesh_query: Query<(Entity, &mut PlanetMeshTag)>,
    mut player_query: Query<
        (Entity, &mut Transform),
        With<MyPlayerTag>,
    >,
) {
    let rectangle = RRectangle::new(Vec2::new(0.5, 1.));
    let circle = RCircle::new(0.3, 20);

    let rect_mesh = meshes.add(LineStrip {
        points: rectangle.points,
    });

    let circle_mesh = meshes.add(LineStrip {
        points: circle.points,
    });

    let legs_mesh = meshes.add(LineList {
        vertices: vec![
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.0, -0.5, 0.0),
        ],
    });

    let legs_mesh = cmd
        .spawn(MaterialMeshBundle {
            mesh: legs_mesh,
            transform: Transform::from_xyz(0.0, -0.89, 0.0)
                .with_scale(Vec3::new(1., 1.1, 1.)),
            material: line_materials.add(LineMaterial {
                color: Color::rgb(5.0, 1.0, 1.0),
            }),
            ..Default::default()
        })
        .insert(PlayerMeshTag)
        .insert(Name::new("legs"))
        .id();

    let body_mesh = cmd
        .spawn(MaterialMeshBundle {
            mesh: rect_mesh.clone(),
            transform: Transform::from_xyz(0.0, 0.1, 0.0)
                .with_scale(Vec3::new(0.9, 1., 1.)),
            material: line_materials.add(LineMaterial {
                color: Color::rgb(5.0, 1.0, 1.0),
            }),
            ..Default::default()
        })
        .insert(PlayerMeshTag)
        .insert(Name::new("body"))
        .id();

    let jet_pack_mesh = cmd
        .spawn(MaterialMeshBundle {
            mesh: rect_mesh.clone(),
            transform: Transform::from_xyz(-0.45, 0.3, 0.0)
                .with_scale(Vec3::new(0.9, 0.7, 1.)),
            material: line_materials.add(LineMaterial {
                color: Color::rgb(5.0, 1.0, 1.0),
            }),
            ..Default::default()
        })
        .insert(PlayerMeshTag)
        .insert(Name::new("jet pack"))
        .id();

    let head_mesh = cmd
        .spawn(MaterialMeshBundle {
            mesh: circle_mesh,
            transform: Transform::from_xyz(0.0, 1.1, 0.0)
                .with_scale(Vec3::new(1.2, 1.2, 1.)),
            material: line_materials.add(LineMaterial {
                color: Color::rgb(5.0, 1.0, 1.0),
            }),
            ..Default::default()
        })
        .insert(PlayerMeshTag)
        .insert(Name::new("head"))
        .id();

    let meshes_parent = cmd
        .spawn(SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(0.35, 0.35, 0.35)),
        ))
        .insert(Name::new("meshes"))
        .insert(PlayerMeshesParentTag)
        .id();

    cmd.entity(meshes_parent).push_children(&[
        body_mesh,
        head_mesh,
        jet_pack_mesh,
        legs_mesh,
    ]);

    let entity = player_query.get_single_mut().unwrap().0;
    cmd.entity(entity).push_children(&[meshes_parent]);
}

trait noo {
    fn noo(x: f32, y: f32) -> Vec3;
}

impl noo for Vec3 {
    fn noo(x: f32, y: f32) -> Vec3 {
        Vec3::new(x, y, 0.0)
    }
}

fn spawn_rocket_ystem(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    // mut mesh_query: Query<(Entity, &mut PlanetMeshTag)>,
    mut player_query: Query<
        (Entity, &mut Transform),
        With<MyPlayerTag>,
    >,
) {
    let lines = vec![
        Vec3::noo(-1.0, -1.0),
        Vec3::noo(-1.0, 1.0),
        Vec3::noo(0.0, -1.0),
        Vec3::noo(0.0, 1.0),
        Vec3::noo(1.0, -1.0),
        Vec3::noo(1.0, 1.0),
    ];

    let rocket_mesh =
        meshes.add(LineList { vertices: lines });
    let rocket_material =
        line_materials.add(LineMaterial {
            color: Color::rgb(1.0, 1.0, 5.0),
        });

    let rocket_entity = cmd
        .spawn(MaterialMeshBundle {
            mesh: rocket_mesh.clone(),
            transform: Transform::from_xyz(0.18, -0.2, 0.0)
                .with_scale(Vec3::noo(0.04, 0.15)),
            material: rocket_material,
            ..Default::default()
        })
        .insert(RocketTag)
        .insert(Name::new("rocket"))
        .id();

    let rocket_parent_entity = cmd
        .spawn(SpatialBundle::default())
        .insert(Name::new("rocket"))
        .insert(RocketParentTag)
        .id();

    cmd.entity(rocket_parent_entity)
        .push_children(&[rocket_entity]);

    let player_entity =
        player_query.get_single_mut().unwrap().0;

    cmd.entity(player_entity)
        .push_children(&[rocket_parent_entity]);
}

fn update_rocket_mesh_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut rocket_query: Query<
        (Entity, &mut Handle<Mesh>),
        With<RocketTag>,
    >,
    mut rocket_transform_query: Query<
        &mut Transform,
        With<RocketParentTag>,
    >,
    mut rocket_visibility_query: Query<
        &mut Visibility,
        With<RocketParentTag>,
    >,
    direction: Query<(&Direction), With<MyPlayerTag>>,
    rocket_status: Res<RocketStatusResource>,
) {
    for (entity, mut mesh_handle) in rocket_query.iter_mut()
    {
        let mut rng = rand::thread_rng();
        let mut ro = || rng.gen_range(-0.5..0.5);

        let mut visibility =
            rocket_visibility_query.single_mut();

        let r = &rocket_status.status;

        match rocket_status.status {
            RocketStatus::Firing => {
                *visibility = Visibility::Visible;

                let lines = vec![
                    Vec3::noo(-1.0 + ro(), -1.0 + ro()),
                    Vec3::noo(-1.0 + ro(), 1.0 + ro()),
                    Vec3::noo(0.0 + ro(), -1.0 + ro()),
                    Vec3::noo(0.0 + ro(), 1.0 + ro()),
                    Vec3::noo(1.0 + ro(), -1.0 + ro()),
                    Vec3::noo(1.0 + ro(), 1.0 + ro()),
                ];

                let mut rocket_transform =
                    rocket_transform_query.single_mut();
                let x = rocket_transform.scale.x.abs();

                if let Ok(d) = direction.get_single() {
                    match d.left_right {
                        LeftRight::Left => {
                            rocket_transform.scale.x = x;
                        }
                        LeftRight::Right => {
                            rocket_transform.scale.x = -x;
                        }
                    }
                }

                let new_rocket_mesh = meshes
                    .add(LineList { vertices: lines });
                *mesh_handle = new_rocket_mesh;
            }
            RocketStatus::NotFiring => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn set_player_direction(
    mut meshes_parent_query: Query<
        &mut Transform,
        With<PlayerMeshesParentTag>,
    >,
    direction: Query<(&Direction), With<MyPlayerTag>>,
) {
    if let Ok(mut meshes_parent_transform) =
        meshes_parent_query.get_single_mut()
    {
        let x = meshes_parent_transform.scale.x.abs();

        if let Ok(d) = direction.get_single() {
            match d.left_right {
                LeftRight::Left => {
                    meshes_parent_transform.scale.x = -x;
                }
                LeftRight::Right => {
                    meshes_parent_transform.scale.x = x;
                }
            }
        }
    }
}

fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ui_state::UiState>,
    mut general_update_event_writer: EventWriter<GeneralUpdateEvent>,
    mut player_event_writer: EventWriter<PlayerEvent>,
    mut regenerate_event_writer: EventWriter<RegeneratePlanetEvent>,
) {
    if keyboard_input.pressed(KeyCode::Digit1) {
        state.camera_mode =
            CameraMode::TexturePlanetOverview;
        state.show_texture = true;
        state.show_vectors = false;
        general_update_event_writer.send(GeneralUpdateEvent{});
    }
    if keyboard_input.pressed(KeyCode::Digit2) {
        state.camera_mode =
            CameraMode::VectorPlanetOverview;
        state.show_texture = false;
        state.show_vectors = true;
        general_update_event_writer.send(GeneralUpdateEvent{});
    }
    if keyboard_input.pressed(KeyCode::Digit3) {
        state.camera_mode = CameraMode::BothOverview;
        state.show_texture = true;
        state.show_vectors = true;
        general_update_event_writer.send(GeneralUpdateEvent{});
    }
    if keyboard_input.pressed(KeyCode::Digit4) {
        state.camera_mode = CameraMode::Player;
        state.show_texture = false;
        state.show_vectors = true;
        general_update_event_writer.send(GeneralUpdateEvent{});
    }
    if keyboard_input.pressed(KeyCode::Digit9) {
        state.camera_mode = CameraMode::VectorPlanetOverview;
        state.show_texture = false;
        state.show_vectors = true;
        state.radius = 1.;
        state.resolution = 575;
        state.brush_size = 0.45;

        regenerate_event_writer.send(RegeneratePlanetEvent{ ui_state: state.clone() });
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        player_event_writer.send(crate::player::PlayerEvent{event_type: crate::player::PlayerEventType::Respawn, ui_state: state.clone()});
    }
}

fn user_input(
    mut direction_query: Query<
        &mut Direction,
        With<MyPlayerTag>,
    >,
    mut query: Query<
        (&mut Transform, &mut ExternalForce),
        With<MyPlayerTag>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ui_state: Res<ui_state::UiState>,
    mut rocket_status: ResMut<RocketStatusResource>,
) {
    let move_force = ui_state.player_move_force;
    let jetpack_force = ui_state.player_jetpack_force;

    for (t, mut ef) in query.iter_mut() {
        rocket_status.status = RocketStatus::NotFiring;

        ef.force = Vec2::new(0.0, 0.0);

        let grav_scale = 5.;
        ef.force = -t.translation.into_vec2().normalize()
            * grav_scale;

        let direction =
            t.translation.truncate().normalize(); // Normalize to get direction
        let perp_clockwise =
            Vec2::new(-direction.y, direction.x); // Rotate 90 degrees clockwise
        let perp_counter_clockwise =
            Vec2::new(direction.y, -direction.x); // Rotate 90 degrees counter-clockwise

        if keyboard_input.pressed(KeyCode::KeyW) {
            ef.force += direction * jetpack_force;
            rocket_status.status = RocketStatus::Firing;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            ef.force -= direction * move_force;
            rocket_status.status = RocketStatus::Firing;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            ef.force += perp_clockwise * move_force;
            for mut d in direction_query.iter_mut() {
                d.left_right = LeftRight::Left;
            }
            rocket_status.status = RocketStatus::Firing;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            ef.force += perp_counter_clockwise * move_force;
            for mut d in direction_query.iter_mut() {
                d.left_right = LeftRight::Right;
            }
            rocket_status.status = RocketStatus::Firing;
        }
    }
}

fn reset_player_system(
    mut event_reader: EventReader<PlayerEvent>,
    mut query: Query<&mut Transform, With<MyPlayerTag>>,
    ui_state: Res<ui_state::UiState>,
) {
    for e in event_reader.read() {
        match e.event_type {
            PlayerEventType::Respawn => {
                for mut transform in query.iter_mut() {
                    // Reset the player position here
                    let scale = ui_state.scale;
                    let radius = ui_state.radius;
                    transform.translation =
                        get_spawn_point(scale, radius);
                }
            }
            _ => {}
        }
    }
}

fn apply_angle(
    mut q: Query<&mut Transform, With<MyPlayerTag>>,
    ui_state: Res<ui_state::UiState>,
) {
    let mut transform = q.single_mut();

    let distance_to_center = distance_squared(
        &Vec2::new(
            transform.translation.x,
            transform.translation.y,
        ),
        &Vec2::new(0.0, 0.0),
    );

    let t = (distance_to_center / 100.).clamp(0., 1.);
    let direction = transform.translation - Vec3::ZERO; // Vec3::ZERO is the origin
    let angle = f32::atan2(direction.x, direction.y);
    let target_rotatin = Quat::from_rotation_z(-angle);
    let current_rotation = transform.rotation;
    let new_rotation = current_rotation.slerp(
        target_rotatin,
        ui_state.player_rotate_force,
    );
    transform.rotation =
        current_rotation.lerp(new_rotation, t);
}

pub fn distance_squared(t1: &Vec2, t2: &Vec2) -> f32 {
    let x = t1.x - t2.x;
    let y = t1.y - t2.y;
    x * x + y * y
}
