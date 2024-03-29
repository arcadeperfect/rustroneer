use bevy::{prelude::*, utils::tracing};
use bevy_rapier2d::prelude::*;
use planet::{planet_data::PlanetData, PlanetBuilder, PlanetOptions};


use crate::{
    line::{LineList, LineMaterial},
    ui::UiChangedEvent,
    ui_state::{BitmapDisplay, UiState},
};

pub struct PlanetPlugin;

use super::conversions::*;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<LineMaterial>::default())
            .insert_resource(PlanetBuilderResource {
                builder: PlanetBuilder::new(0),
            })
            .insert_resource(TexturePlaneEntityResource { entity: None })
            .add_systems(Startup, spawn_planet_root_system)
            .add_systems(PostStartup, rebuild_planet_system)
            .add_systems(Update, rebuild_planet_system)
            .add_systems(Update, spawn_planet_mesh_system)
            .add_systems(Update, spawn_planet_colliders_system)
            .add_systems(Update, update_planet_root_system)
            // .add_systems(Update, update_planet_texture_transform)
            .add_systems(PostStartup, spawn_planet_map_visualiser_system)
            .add_systems(Update, update_planet_texture);
    }
}

#[derive(Resource)]
struct TexturePlaneEntityResource {
    entity: Option<Entity>,
}

#[derive(Component)]
struct NeedsMeshUpdate;

#[derive(Component)]
struct NeedsColliderUpdate;

#[derive(Component)]
struct NeedsTextureUpdate;

#[derive(Component)]
struct PlanetColliderTag;

#[derive(Resource)]
pub struct PlanetBuilderResource {
    pub builder: PlanetBuilder,
}

#[derive(Component)]
pub struct BevyPlanet {
    pub planet_data: Option<PlanetData>,
}

#[derive(Component)]
pub struct PlanetMeshTag;

#[derive(Component)]
pub struct PlanetRootTag;

#[derive(Component)]
pub struct TexturePlanetRootTag;

#[derive(Resource)]
pub struct TexturePlanetEntity {
    pub entity: Option<Entity>,
}

fn spawn_planet_root_system(mut commands: Commands, state: ResMut<UiState>) {
    let planet = None;

    let scale = state.scale;

    commands
        .spawn(SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(scale, scale, 1.)),
        ))
        .insert(Name::new("planet"))
        .insert(BevyPlanet {
            planet_data: planet,
        })
        .insert(PlanetRootTag);
}

fn update_planet_root_system(
    mut query: Query<&mut Transform, With<PlanetRootTag>>,
    state: Res<UiState>,
    mut events: EventReader<UiChangedEvent>,
    mut vis_query: Query<&mut Visibility, With<PlanetRootTag>>,
) {
    let ui_event = events.read();
    for _event in ui_event {
        let mut vis = vis_query.single_mut();

        match state.show_vectors {
            true => {
                *vis = Visibility::Visible;
            }
            false => {
                *vis = Visibility::Hidden;
            }
        }

        if let Ok(mut transform) = query.get_single_mut() {
            transform.scale = Vec3::new(state.scale, state.scale, 1.);
        }
    }
}

fn rebuild_planet_system(
    mut commands: Commands,
    builder_resource: ResMut<PlanetBuilderResource>,
    mut events: EventReader<UiChangedEvent>,
    mut planet_query: Query<(Entity, &mut BevyPlanet), With<Name>>,
) {
    if events.is_empty() {
        return;
    }

    let ui_event = events.read();
    for event in ui_event {
        let state = event.ui_state.clone();
        let options = PlanetOptions::from(event.ui_state.clone());
        let planet_data = builder_resource
            .builder
            .build(options, state.noise.iter().collect());
        match planet_data {
            Ok(planet) => {
                if let Ok((entity, mut bevy_planet)) = planet_query.get_single_mut() {
                    bevy_planet.planet_data = Some(planet);
                    commands.entity(entity).insert(NeedsMeshUpdate);
                    commands.entity(entity).insert(NeedsColliderUpdate);
                    commands.entity(entity).insert(NeedsTextureUpdate);
                }
            }
            Err(err) => {
                tracing::error!("error building planet: {}", err);
                if let Ok((_entity, mut bevy_planet)) = planet_query.get_single_mut() {
                    bevy_planet.planet_data = None;
                }
            }
        }
    }
    events.clear();
}

fn spawn_planet_mesh_system(
    planet_query: Query<(&BevyPlanet, &NeedsMeshUpdate), With<Name>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut mesh_query: Query<(Entity, &mut PlanetMeshTag)>,
    mut planet_root_query: Query<(Entity, &mut PlanetRootTag)>,
) {
    for (bevy_planet, _) in planet_query.iter() {
        if let Some(planet) = bevy_planet.planet_data.as_ref() {
            for (mesh_entity, _) in mesh_query.iter_mut() {
                commands.entity(mesh_entity).despawn();
            }
            let lines = planet.get_line_list();
            let m = meshes.add(LineList { vertices: lines });

            let mesh_child = commands
                .spawn(MaterialMeshBundle {
                    mesh: m,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1., 1., 1.)),
                    material: line_materials.add(LineMaterial {
                        color: Color::GREEN,
                    }),
                    ..Default::default()
                })
                .insert(PlanetMeshTag)
                .id();

            if let Ok((entity, _)) = planet_root_query.get_single_mut() {
                commands.entity(entity).push_children(&[mesh_child]);
                commands.entity(entity).remove::<NeedsMeshUpdate>();
            }
        }
    }
}

fn spawn_planet_colliders_system(
    mut commands: Commands,
    planet_query: Query<(Entity, &BevyPlanet, &NeedsColliderUpdate)>,
    collider_query: Query<Entity, With<PlanetColliderTag>>,
) {
    for (planet_entity, planet, _needs_update) in planet_query.iter() {
        if let Some(planet) = planet.planet_data.as_ref() {
            for entity in collider_query.iter() {
                commands.entity(entity).despawn();
            }

            let colliders = get_colliders(&planet.polylines);

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
            commands
                .entity(planet_entity)
                .remove::<NeedsColliderUpdate>();
        }
    }
}

fn get_colliders(vecs: &Vec<Vec<Vec2>>) -> Vec<Collider> {
    let mut colliders = Vec::new();
    for vec in vecs {
        colliders.push(Collider::polyline(vec.clone(), None));
    }

    return colliders;
}

fn spawn_planet_map_visualiser_system(
    state: ResMut<UiState>,
    planet_query: Query<(Entity, &BevyPlanet), With<Name>>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_plane_entity_resource: ResMut<TexturePlaneEntityResource>,
    mut planet_root_query: Query<(Entity, &mut PlanetRootTag)>,
) {
    let _scale = state.scale;

    for (planet_entity, bevy_planet) in planet_query.iter() {
        if let Some(planet) = bevy_planet.planet_data.as_ref() {
            let main_map = &planet.planet_map.main; 
            
                let texture_planet_material: Handle<StandardMaterial> =
                    materials.add(StandardMaterial {
                        base_color_texture: Some(images.add(umap_to_bevy_image(&main_map))),
                        unlit: true,
                        ..default()
                    });

                let texture_planet_mesh = meshes.add(Plane3d::default());
                let texture_planet_bundle: MaterialMeshBundle<StandardMaterial> = PbrBundle {
                    mesh: texture_planet_mesh,
                    material: texture_planet_material.clone(),
                    transform: Transform::from_xyz(-1.7, 0.0, -0.2)
                        .with_rotation(Quat::from_euler(
                            EulerRot::XYZ,
                            std::f32::consts::PI / 2.,
                            0.,
                            0.,
                        ))
                        .with_scale(Vec3::new(1., 1., 1.)),
                    ..default()
                };

                let texture_plane_entity = commands
                    .spawn(texture_planet_bundle)
                    .insert(TexturePlanetRootTag)
                    .id();
                texture_plane_entity_resource.entity = Some(texture_plane_entity);

                if let Ok((entity, _)) = planet_root_query.get_single_mut() {
                    commands
                        .entity(entity)
                        .push_children(&[texture_plane_entity]);
                }
            

            commands
                .entity(planet_entity)
                .remove::<NeedsTextureUpdate>();
        }
    }
}

fn update_planet_texture(
    state: ResMut<UiState>,
    texture_planet_entity_resource: Res<TexturePlaneEntityResource>,
    planet_query: Query<(Entity, &BevyPlanet, &NeedsTextureUpdate), With<Name>>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Handle<StandardMaterial>>,
    mut vis_query: Query<&mut Visibility, With<TexturePlanetRootTag>>,
) {
    for (planet_entity, bevy_planet, _) in planet_query.iter() {
        if let Ok(mut vis) = vis_query.get_single_mut() {
            match state.show_texture {
                true => *vis = Visibility::Visible,
                false => *vis = Visibility::Hidden,
            }

            if let Some(planet) = bevy_planet.planet_data.as_ref() {
                if let Some(texture_plane_entity) = texture_planet_entity_resource.entity {
                    if let Ok(mut material_handle) = query.get_mut(texture_plane_entity) {
                        let new_image = match state.bitmap_dislpay {
                            BitmapDisplay::PlanetRaw => umap_to_bevy_image(&planet.planet_map.main),
                            
                            // match &planet.planet_map.main {
                            //     Some(main_map) => umap_to_bevy_image(main_map),
                            //     None => continue,
                            // },

                            BitmapDisplay::PlanetProcessed => imagebuffer_to_bevy_image(&planet.image),

                            // BitmapDisplay::PlanetProcessed => match &planet.image {
                            //     Some(image) => imagebuffer_to_bevy_image(image),
                            //     None => continue,
                            // },

                            BitmapDisplay::Altitude => fmap_to_bevy_image(&planet.planet_map.altitude ),


                            // BitmapDisplay::Altitude => match &planet.planet_map.altitude {
                            //     Some(v) => fmap_to_bevy_image(v),
                            //     None => continue,
                            // },

                            BitmapDisplay::Depth => fmap_to_bevy_image(&planet.planet_map.depth ),

                            // BitmapDisplay::Depth => match &planet.planet_map.depth {
                            //     Some(v) => fmap_to_bevy_image(v),
                            //     None => continue,
                            // },
                            BitmapDisplay::Mask => match &planet.planet_map.mask {
                                Some(v) => fmap_to_bevy_image(v),
                                None => continue,
                            },
                            BitmapDisplay::RoomsRaw => match &planet.planet_map.rooms_raw {
                                Some(v) => umap_to_bevy_image(v),
                                None => continue,
                            },
                            BitmapDisplay::RoomsDebug => match &planet.roooms {
                                // Some(rooms) => imagebuffer_to_bevy_image(rooms_debug),
                                Some(_t) => {
                                    continue;
                                    // if let Some(rooms) = t.rooms{
                                    //     room_vec_to_bevy_image(rooms, planet.get_dimension().unwrap())    
                                    // }
                                    // else {
                                    //     continue;
                                    // }
                                }
                                None => continue,
                            },

                            BitmapDisplay::TileMapDebug => tile_map_to_bevy_image(&planet.tile_map),

                            // BitmapDisplay::TileMapDebug => match &planet.tile_map {
                            //     // Some(rooms) => imagebuffer_to_bevy_image(rooms_debug),
                            //     Some(t) => tile_map_to_bevy_image(t),
                            //     None => continue,
                            // },
                        };

                        let new_image_handle = images.add(new_image);
                        let new_material = StandardMaterial {
                            base_color_texture: Some(new_image_handle),
                            unlit: true,
                            ..default()
                        };

                        *material_handle = materials.add(new_material);
                    }
                }

                commands
                    .entity(planet_entity)
                    .remove::<NeedsTextureUpdate>();
            }
        }
    }
}
