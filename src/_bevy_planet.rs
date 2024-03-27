// use anyhow::Result;
// use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
// use planet::{PlanetBuilder, PlanetData, PlanetOptions};

// use crate::{
//     line::{LineList, LineMaterial},
//     types::UiState,
//     ui::SliderChangedEvent,
// };

// pub struct PlanetPlugin;

// impl Plugin for PlanetPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(MaterialPlugin::<LineMaterial>::default())
//             .insert_resource(PlanetBuilderResource {
//                 builder: PlanetBuilder::new(0),
//             })
//             .insert_resource(PlanetSpawnerResource {})
//             .add_systems(PostStartup, draw_lines);
//     }
// }


// #[derive(Component)]
// pub struct BevyPlanet {
//     pub planet: PlanetData,
// }

// #[derive(Component)]
// pub struct PlanetMeshTag;

// #[derive(Resource)]
// pub struct PlanetBuilderResource {
//     pub builder: PlanetBuilder,
// }

// #[derive(Resource)]
// pub struct PlanetSpawnerResource {}

// impl PlanetSpawnerResource {
//     pub fn spawn_planet_root(
//         mut commands: Commands,
//         builder_resource: ResMut<PlanetBuilderResource>,
//         planet_options: PlanetOptions,
//     ) {
//         let scale = planet_options.planet_scale;

//         let parent = commands
//         .spawn(SpatialBundle::from_transform(
//             Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(scale, scale, 1.)),
//         ))
//         .insert(Name::new("Planet"))
//         .insert(BevyPlanet { planet })
//         .id();
//     }
// }




// fn InitPlanet(
//     builder_resource: ResMut<PlanetBuilderResource>,
//     spawner_resource: ResMut<PlanetSpawnerResource>,
// ) {
// }

// // fn SpawnPlanetRoot(
// //     mut commands: Commands,
// //     builder_resource: ResMut<PlanetBuilderResource>,
// //     planet_options: PlanetOptions,

// // ) {
// //     let planet: PlanetData = builder_resource.builder.build(planet_options);
// // }

// fn temp(
//     mut commands: Commands,
//     builder_resource: ResMut<PlanetBuilderResource>,
//     planet_options: PlanetOptions,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut line_materials: ResMut<Assets<LineMaterial>>,
// ) {
//     // let planet_options = PlanetOptions::from(event.ui_state.clone());
//     let planet: Planet = builder_resource.builder.build(planet_options).unwrap();
//     let verts = planet.get_line_list();
//     let colliders = get_colliders(&planet.poly_lines);

//     let parent = commands
//         .spawn(SpatialBundle::from_transform(
//             Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(5., 5., 1.)),
//         ))
//         .insert(Name::new("Planet"))
//         .insert(BevyPlanet { planet })
//         .id();

//     let mut childs = Vec::new();

//     for collider in colliders {
//         childs.push(
//             commands
//                 .spawn(collider.clone())
//                 .insert(TransformBundle::from(Transform::default()))
//                 .insert(Name::new("Collider"))
//                 .id(),
//         )
//     }

//     let m = meshes.add(LineList { vertices: verts });

//     let mesh_child = commands
//         .spawn(MaterialMeshBundle {
//             mesh: m,
//             transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1., 1., 1.)),
//             material: line_materials.add(LineMaterial {
//                 color: Color::WHITE,
//             }),
//             ..Default::default()
//         })
//         .insert(PlanetMeshTag)
//         .id();

//     commands.entity(parent).push_children(&[mesh_child]);
//     commands.entity(parent).push_children(&childs);
// }

// fn spawn_planet(
//     mut commands: Commands,
//     mut line_materials: ResMut<Assets<LineMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut events: EventReader<SliderChangedEvent>,
//     builder_resource: ResMut<PlanetBuilderResource>,
//     planet_query: Query<Entity, With<BevyPlanet>>,
// ) {
//     if events.is_empty() {
//         return;
//     }

//     let ui_event = events.read();
//     for event in ui_event {
//         for planet_entity in planet_query.iter() {
//             commands.entity(planet_entity).despawn_recursive();
//             temp(commands, builder_resource, planet_options, meshes, line_materials)
//         }
//     }
//     events.clear();
// }

// fn rebuild_planet(
//     mut events: EventReader<SliderChangedEvent>,
//     mut builder_resource: ResMut<PlanetBuilderResource>,
// ) {
//     if events.is_empty() {
//         return;
//     }
//     let ui_event = events.read();
//     for event in ui_event {
//         builder_resource.builder = PlanetBuilder::new(event.ui_state.seed);
//     }
//     events.clear();
// }


// fn spawn_planet_root_system(
//     mut commands: Commands,
//     mut builder_resource: ResMut<PlanetBuilderResource>,
//     planet_options: PlanetOptions,
// ) {
//     let parent = commands
//         .spawn(SpatialBundle::from_transform(
//             Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(5., 5., 1.)),
//         ))
//         .insert(Name::new("Planet"))
//         .insert(BevyPlanet { planet })
//         .id();
// }


// fn draw_lines(
//     planet_query: Query<&BevyPlanet>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut query: Query<(&mut Handle<Mesh>, &PlanetMeshTag)>,
// ) {
//     if planet_query.is_empty() {
//         return;
//     }

//     let verts = planet_query.single().planet.get_line_list();
//     let m = meshes.add(LineList { vertices: verts });

//     for (mut mesh_handle, _) in query.iter_mut() {
//         *mesh_handle = m.clone();
//     }
// }

// fn get_colliders(vecs: &Vec<Vec<Vec2>>) -> Vec<Collider> {
//     let mut colliders = Vec::new();
//     for vec in vecs {
//         colliders.push(Collider::polyline(vec.clone(), None));
//     }
//     return colliders;
// }

// impl From<UiState> for PlanetOptions {
//     fn from(ui_state: UiState) -> Self {
//         Self {
//             seed: 0,
//             min_room_size: 20,

//             frequency: ui_state.frequency,
//             amplitude: ui_state.amplitude,
//             radius: ui_state.radius,
//             resolution: ui_state.resolution,
//             thresh: ui_state.thresh,
//             iterations: ui_state.iterations,
//             weight: ui_state.weight,
//             blur: ui_state.blur,
//         }
//     }
// }
