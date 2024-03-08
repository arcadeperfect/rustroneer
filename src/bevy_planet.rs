use anyhow::Result;
use bevy::{ecs::query, prelude::*, transform::commands, utils::tracing};
use bevy_rapier2d::prelude::*;
use planet::{PlanetBuilder, PlanetData, PlanetOptions};

use crate::{
    line::{LineList, LineMaterial},
    types::UiState,
    ui::UiChangedEvent,
};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<LineMaterial>::default())
            .insert_resource(PlanetBuilderResource {
                builder: PlanetBuilder::new(0),
            })
            .add_systems(Startup, (spawn_planet_root_system))
            .add_systems(Update, (build_planet_system))
            .add_systems(Update, spawn_planet_mesh_system)
            .add_systems(Update, (spawn_planet_colliders_system));

    }
}

#[derive(Component)]
struct NeedsMeshUpdate;

#[derive(Component)]
struct NeedsColliderUpdate;

#[derive(Component)]
struct PlanetColliderTag;



#[derive(Resource)]
pub struct PlanetBuilderResource {
    pub builder: PlanetBuilder,
}
#[derive(Component)]
pub struct BevyPlanet {
    pub planet: Option<PlanetData>,
}

#[derive(Component)]
pub struct PlanetMeshTag;

#[derive(Component)]
pub struct PlanetRootTag;

fn spawn_planet_root_system(mut commands: Commands) {
    let planet = None;

    commands
        .spawn(SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1., 1., 1.)),
        ))
        .insert(Name::new("planet"))
        .insert(BevyPlanet { planet })
        .insert(PlanetRootTag);
}

fn build_planet_system(
    mut commands: Commands,
    builder_resource: ResMut<PlanetBuilderResource>,
    mut events: EventReader<UiChangedEvent>,
    // mut planet_query: Query<&mut BevyPlanet, With<Name>>,
    mut planet_query: Query<(Entity, &mut BevyPlanet), With<Name>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut mesh_query: Query<Entity, With<PlanetMeshTag>>,
) {
    if events.is_empty() {
        return;
    }

    let ui_event = events.read();
    for event in ui_event {
        let options = PlanetOptions::from(event.ui_state.clone());

        let planet_data = builder_resource.builder.build(options);

        match planet_data {
            Ok(planet) => {
                if let Ok((entity, mut bevy_planet)) = planet_query.get_single_mut() {
                    bevy_planet.planet = Some(planet);
                    commands.entity(entity).insert(NeedsMeshUpdate);
                    commands.entity(entity).insert(NeedsColliderUpdate);
                }
            }
            Err(err) => {
                tracing::error!("error building planet: {}", err);
                if let Ok((entity, mut bevy_planet)) = planet_query.get_single_mut() {
                    bevy_planet.planet = None;
                }
            }
        }
    }
    events.clear();
}


fn spawn_planet_mesh_system(
    mut commands: Commands,
    planet_query: Query<(&BevyPlanet, &NeedsMeshUpdate), With<Name>>,
    
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut mesh_query: Query<(Entity, &mut PlanetMeshTag)>,
    mut planet_root_query: Query<(Entity, &mut PlanetRootTag)>,
) {
    for (bevy_planet, _) in planet_query.iter() {
        if let Some(planet) = bevy_planet.planet.as_ref() {
            for (mesh_entity, _) in mesh_query.iter_mut() {
                commands.entity(mesh_entity).despawn();
            }
            let lines = planet.get_line_list();
            let m = meshes.add(LineList { vertices: lines });

            println!("spawning mesh");

            let mesh_child = commands
                .spawn(MaterialMeshBundle {
                    mesh: m,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1., 1., 1.)),
                    material: line_materials.add(LineMaterial {
                        color: Color::WHITE,
                    }),
                    ..Default::default()
                })
                .insert(PlanetMeshTag)
                .id();

            // for (planet_root, _) in planet_root_query.iter_mut() {
                
            // }

            if let Ok((entity, _)) = planet_root_query.get_single_mut() {
                commands.entity(entity).push_children(&[mesh_child]);
                commands.entity(entity).remove::<NeedsMeshUpdate>();
            }
        }
    }
}


fn spawn_planet_colliders_system(
    mut commands: Commands,
    mut planet_query: Query<(Entity, &BevyPlanet, &NeedsColliderUpdate)>,
    mut collider_query: Query<Entity, With<PlanetColliderTag>>,
){

    


    for (planet_entity, planet, needs_update) in planet_query.iter() {
        if let Some(planet) = planet.planet.as_ref() {


            for entity in collider_query.iter() {
                commands.entity(entity).despawn();
            }

            let colliders = get_colliders(&planet.poly_lines);

            let mut childs = Vec::new();

            for collider in colliders {
                childs.push(
                    commands
                        .spawn(collider)
                        .insert(TransformBundle::from(Transform::default()))
                        .insert(Name::new("Collider"))
                        .insert(PlanetColliderTag)
                        .id(),
                )
            }       

            commands.entity(planet_entity).push_children(&childs);
            commands.entity(planet_entity).remove::<NeedsColliderUpdate>();
        }
    }
}




fn get_colliders(vecs: &Vec<Vec<Vec2>>) -> Vec<Collider> {
    let mut colliders = Vec::new();
    for vec in vecs {
        colliders.push(Collider::polyline(vec.clone(), None));
    }

    println!("found {:?} colliders", colliders.len());

    return colliders;
}





impl From<UiState> for PlanetOptions {
    fn from(ui_state: UiState) -> Self {
        Self {
            seed: 0,
            min_room_size: 20,

            frequency: ui_state.frequency,
            amplitude: ui_state.amplitude,
            radius: ui_state.radius,
            resolution: ui_state.resolution,
            thresh: ui_state.thresh,
            iterations: ui_state.iterations,
            weight: ui_state.weight,
            blur: ui_state.blur,
        }
    }
}
