use bevy::{ecs::query, prelude::*, utils::tracing};
use bevy_rapier2d::prelude::*;
use planet::{
    planet_data::{
        self, flatten_and_zip, march_squares_rgba,
        PlanetData,
    }, types::PolyLines, PlanetBuilder, PlanetOptions
};

use crate::{
    line::{LineList, LineMaterial},
    ui::{GeneralUpdateEvent, ModifyMeshEvent, MouseClickWorldEvent, RegeneratePlanetEvent},
    ui_state::{BitmapDisplay, UiState},
};

pub struct PlanetPlugin;

use super::conversions::*;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            MaterialPlugin::<LineMaterial>::default(),
        )
        .insert_resource(PlanetBuilderResource {
            builder: PlanetBuilder::new(0),
        })
        .insert_resource(TexturePlaneEntityResource {
            entity: None,
        })
        .insert_resource(Contours{
            contours: None,
        })
        .add_systems(Startup, spawn_planet_root_system)
        .add_systems(PostStartup, rebuild_planet_system)
        .add_systems(Update, rebuild_planet_system)
        .add_systems(Update, spawn_planet_mesh_system)
        .add_systems(Update, spawn_planet_colliders_system)
        .add_systems(Update, update_planet_root_system)
        .add_systems(Update, modify_image_and_refresh_mesh_system)
        // .add_systems(Update, update_planet_texture_transform)
        .add_systems(
            PostStartup,
            spawn_planet_map_visualiser_system,
        )
        .add_systems(Update, update_planet_texture)
        // .add_systems(Update, rremarch);
        .add_systems(Update, refresh_planet_texture);
    }
}

#[derive(Resource)]
pub struct PlanetBuilderResource {
    pub builder: PlanetBuilder,
}
#[derive(Resource)]
pub struct TexturePlanetEntityResource {
    pub entity: Option<Entity>,
}
#[derive(Resource)]
struct TexturePlaneEntityResource {
    entity: Option<Entity>,
}

#[derive(Resource)]
pub struct Contours{
    pub contours: Option<PolyLines>,
}

#[derive(Component)]
struct NeedsMeshUpdate;

#[derive(Component)]
struct NeedsColliderUpdate;

#[derive(Component)]
struct NeedsTextureUpdate;

#[derive(Component)]
struct PlanetColliderTag;

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

fn spawn_planet_root_system(
    mut commands: Commands,
    state: ResMut<UiState>,
) {
    let planet = None;

    let scale = state.scale;

    commands
        .spawn(SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(scale, scale, 1.)),
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
    mut events: EventReader<GeneralUpdateEvent>,
    mut vis_query: Query<
        &mut Visibility,
        With<PlanetRootTag>,
    >,
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
            transform.scale =
                Vec3::new(state.scale, state.scale, 1.);
        }
    }
}

fn rebuild_planet_system(
    mut cmd: Commands,
    builder_resource: ResMut<PlanetBuilderResource>,
    mut events: EventReader<RegeneratePlanetEvent>,
    mut planet_query: Query<
        (Entity, &mut BevyPlanet),
        With<Name>,
    >,
) {
    if events.is_empty() {
        return;
    }

    let ui_event = events.read();
    for event in ui_event {
        let state = event.ui_state.clone();
        let options =
            PlanetOptions::from(event.ui_state.clone());
        let planet_data = builder_resource.builder.build(
            options,
            state.fractal_noises.iter().collect(),
        );
        match planet_data {
            Ok(planet) => {
                if let Ok((entity, mut bevy_planet)) =
                    planet_query.get_single_mut()
                {
                    bevy_planet.planet_data = Some(planet);
                    cmd.entity(entity)
                        .insert(NeedsMeshUpdate);
                    cmd.entity(entity)
                        .insert(NeedsColliderUpdate);
                    cmd.entity(entity)
                        .insert(NeedsTextureUpdate);
                }
            }
            Err(err) => {
                tracing::error!(
                    "error building planet: {}",
                    err
                );
                if let Ok((_entity, mut bevy_planet)) =
                    planet_query.get_single_mut()
                {
                    bevy_planet.planet_data = None;
                }
            }
        }
    }
    events.clear();
}

fn modify_image_and_refresh_mesh_system(
    mut cmd: Commands,
    mut planet_query: Query<
        (Entity, &mut BevyPlanet),
        With<Name>,
    >,
    mut events: EventReader<MouseClickWorldEvent>,
    ui_state: Res<UiState>,
    
) {
    
    for event in events.read() {
        
        if let Ok((entity, mut bevy_planet)) =
            planet_query.get_single_mut()
        {
            if let Some(d) = &mut bevy_planet.planet_data {
                // d.image.fill(255);
                let s = ui_state.scale;
                let r = ui_state.resolution;

                let mut scaled_pos = event.pos /s /2.;
                scaled_pos += Vec3::new(0.5, 0.5,0.);
                scaled_pos *= r as f32;
                let x = scaled_pos.x as i32;
                let y = (r as f32 - scaled_pos.y) as i32;

                let brush_radius = (0.006 * r as f32 * (ui_state.brush_size * 2.)) as i32;




                match event.button {
                    MouseButton::Left => {
                        paint(&mut d.image, x, y, brush_radius, 0);
                        cmd.entity(entity).insert(NeedsMeshUpdate);
                        cmd.entity(entity).insert(NeedsColliderUpdate);
                        cmd.entity(entity).insert(NeedsTextureUpdate);
                    }
                    MouseButton::Right => {
                        paint(&mut d.image, x, y, brush_radius, 255);
                        cmd.entity(entity).insert(NeedsMeshUpdate);
                        cmd.entity(entity).insert(NeedsColliderUpdate);
                        cmd.entity(entity).insert(NeedsTextureUpdate);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn refresh_planet_texture(
    mut cmd: Commands,
    mut events: EventReader<GeneralUpdateEvent>,
    mut planet_query: Query<
        (Entity, &mut BevyPlanet),
        With<Name>,
    >,
) {
    for event in events.read() {
        if let Ok((entity, mut bevy_planet)) =
            planet_query.get_single_mut()
        {
            // cmd.entity(entity).insert(NeedsMeshUpdate);
            // cmd.entity(entity).insert(NeedsColliderUpdate);
            cmd.entity(entity).insert(NeedsTextureUpdate);
        }
    }
}

fn spawn_planet_mesh_system(
    planet_query: Query<
        (&BevyPlanet, &NeedsMeshUpdate),
        With<Name>,
    >,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut mesh_query: Query<(Entity, &mut PlanetMeshTag)>,
    mut planet_root_query: Query<(
        Entity,
        &mut PlanetRootTag,
    
    )>,
    mut contours: ResMut<Contours>,
) {
    for (bevy_planet, _) in planet_query.iter() {
        //todo separate respawning from refreshing mesh

        if let Some(planet) =
            bevy_planet.planet_data.as_ref()
        {
            for (mesh_entity, _) in mesh_query.iter_mut() {
                cmd.entity(mesh_entity).despawn();
            }

            let lines_result =
                march_squares_rgba(&planet.image);

            

            if let Ok(liness) = (lines_result) {

                contours.contours = Some(liness);

                let lines = flatten_and_zip(contours.contours.as_ref().unwrap());
                let m = meshes
                    .add(LineList { vertices: lines });

                let mesh_child = cmd
                    .spawn(MaterialMeshBundle {
                        mesh: m,
                        transform: Transform::from_xyz(
                            0.0, 0.0, 0.0,
                        )
                        .with_scale(Vec3::new(1., 1., 1.)),
                        material: line_materials.add(
                            LineMaterial {
                                // color: Color::GREEN,
                                color: Color::rgb(
                                    1.0, 5.0, 3.0,
                                ),
                            },
                        ),
                        ..Default::default()
                    })
                    .insert(PlanetMeshTag)
                    .id();

                if let Ok((entity, _)) =
                    planet_root_query.get_single_mut()
                {
                    cmd.entity(entity)
                        .push_children(&[mesh_child]);
                    cmd.entity(entity)
                        .remove::<NeedsMeshUpdate>();
                }
            }
        }
    }
}

fn spawn_planet_colliders_system(
    mut commands: Commands,
    planet_query: Query<(
        Entity,
        &BevyPlanet,
        &NeedsColliderUpdate,
    )>,
    collider_query: Query<Entity, With<PlanetColliderTag>>,
    polylines: Res<Contours>,
) {
    for (planet_entity, planet, _needs_update) in
        planet_query.iter()
    {
        if let Some(planet) = planet.planet_data.as_ref() {
            for entity in collider_query.iter() {
                commands.entity(entity).despawn();
            }

            // let colliders = get_colliders(&planet.polylines);
            // let colliders = get_colliders(&planet.get_polylines().unwrap());

            // let lines_result =
            //     march_squares_rgba(&planet.image);
                
            if let Some(lines) = polylines.contours.as_ref() {
                let colliders = get_colliders(&lines);

                let mut childs = Vec::new();

                for collider in colliders {
                    childs.push(
                        commands
                            .spawn(collider)
                            .insert(TransformBundle::from(
                                Transform::default(),
                            ))
                            .insert(Name::new("Collider"))
                            .insert(PlanetColliderTag)
                            .id(),
                    )
                }

                commands
                    .entity(planet_entity)
                    .push_children(&childs);
                commands
                    .entity(planet_entity)
                    .remove::<NeedsColliderUpdate>();
            }
        }
    }
}

fn get_colliders(vecs: &Vec<Vec<Vec2>>) -> Vec<Collider> {
    let mut colliders = Vec::new();
    for vec in vecs {
        colliders
            .push(Collider::polyline(vec.clone(), None));
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
    mut texture_plane_entity_resource: ResMut<
        TexturePlaneEntityResource,
    >,
    mut planet_root_query: Query<(
        Entity,
        &mut PlanetRootTag,
    )>,
) {
    let _scale = state.scale;

    for (planet_entity, bevy_planet) in planet_query.iter()
    {
        if let Some(planet) =
            bevy_planet.planet_data.as_ref()
        {
            let main_map = &planet.planet_map.main;

            let texture_planet_material: Handle<
                StandardMaterial,
            > = materials.add(StandardMaterial {
                base_color_texture: Some(
                    images
                        .add(umap_to_bevy_image(&main_map)),
                ),
                unlit: true,
                ..default()
            });

            let texture_planet_mesh =
                meshes.add(Plane3d::default());
            let texture_planet_bundle: MaterialMeshBundle<
                StandardMaterial,
            > = PbrBundle {
                mesh: texture_planet_mesh,
                material: texture_planet_material.clone(),
                transform: Transform::from_xyz(
                    -1.7, 0.0, -0.2,
                )
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
            texture_plane_entity_resource.entity =
                Some(texture_plane_entity);

            if let Ok((entity, _)) =
                planet_root_query.get_single_mut()
            {
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
    texture_planet_entity_resource: Res<
        TexturePlaneEntityResource,
    >,
    planet_query: Query<
        (Entity, &BevyPlanet, &NeedsTextureUpdate),
        With<Name>,
    >,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Handle<StandardMaterial>>,
    mut vis_query: Query<
        &mut Visibility,
        With<TexturePlanetRootTag>,
    >,
) {
    for (planet_entity, bevy_planet, _) in
        planet_query.iter()
    {
        if let Ok(mut vis) = vis_query.get_single_mut() {
            match state.show_texture {
                true => *vis = Visibility::Visible,
                false => *vis = Visibility::Hidden,
            }

            if let Some(planet) =
                bevy_planet.planet_data.as_ref()
            {
                if let Some(texture_plane_entity) =
                    texture_planet_entity_resource.entity
                {
                    if let Ok(mut material_handle) =
                        query.get_mut(texture_plane_entity)
                    {
                        let new_image = match state.bitmap_dislpay {
                            BitmapDisplay::PlanetRaw => umap_to_bevy_image(&planet.planet_map.main),

                            // match &planet.planet_map.main {
                            //     Some(main_map) => umap_to_bevy_image(main_map),
                            //     None => continue,
                            // },
                            BitmapDisplay::PlanetProcessed => {
                                imagebuffer_to_bevy_image(&planet.image)
                            }

                            // BitmapDisplay::PlanetProcessed => match &planet.image {
                            //     Some(image) => imagebuffer_to_bevy_image(image),
                            //     None => continue,
                            // },
                            BitmapDisplay::Altitude => {
                                fmap_to_bevy_image(&planet.planet_map.altitude)
                            }

                            // BitmapDisplay::Altitude => match &planet.planet_map.altitude {
                            //     Some(v) => fmap_to_bevy_image(v),
                            //     None => continue,
                            // },
                            BitmapDisplay::Depth => fmap_to_bevy_image(&planet.planet_map.depth),

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

                        let new_image_handle =
                            images.add(new_image);
                        let new_material =
                            StandardMaterial {
                                base_color_texture: Some(
                                    new_image_handle,
                                ),
                                unlit: true,
                                ..default()
                            };

                        *material_handle =
                            materials.add(new_material);
                    }
                }

                commands
                    .entity(planet_entity)
                    .remove::<NeedsTextureUpdate>();
            }
        }
    }
}


use image::{ImageBuffer, Rgba};

// fn paint(
//     mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
//     x: i32,
//     y: i32,
//     radius: i32,
//     v: u8
// ) {
//     let (width, height) = image.dimensions();

//     for y_coord in 0..height {
//         for x_coor in 0..width {
//             let dx = x_coor as i32 - x;
//             let dy = y_coord as i32 - y;
//             let distance_squared = dx * dx + dy * dy;

//             if distance_squared <= radius * radius {
//                 image.put_pixel(x_coor, y_coord, Rgba([v, v, v, v]));
//             }
//         }
//     }
// }

use std::cmp::min;

fn paint(mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32, radius: i32, v: u8) {
    let (width, height) = image.dimensions();

    // Define the thickness of the antialiasing edge.
    let aa_edge = 3.0; // The thickness of the antialiasing in pixels.

    for y_coord in 0..height {
        for x_coord in 0..width {
            let dx = x_coord as f32 - x as f32;
            let dy = y_coord as f32 - y as f32;
            let distance = (dx * dx + dy * dy).sqrt();
            let radius_f = radius as f32;

            // Check if we're within the antialiasing edge.
            if distance <= radius_f + aa_edge && distance >= radius_f - aa_edge {
                let alpha = ((radius_f + aa_edge - distance) / (2.0 * aa_edge)).clamp(0.0, 1.0);
                let existing_color = image.get_pixel(x_coord, y_coord).0;
                let blended_color = blend(existing_color, [v, v, v, (255.0 * 0.2 * alpha) as u8]);
                image.put_pixel(x_coord, y_coord, Rgba(blended_color));
            } else if distance < radius_f - aa_edge {
                // Inside the circle, fully opaque
                image.put_pixel(x_coord, y_coord, Rgba([v, v, v, 255]));
            }
        }
    }
}

// Blend two colors together based on alpha
fn blend(color1: [u8; 4], color2: [u8; 4]) -> [u8; 4] {
    let alpha = color2[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;
    [
        (color1[0] as f32 * inv_alpha + color2[0] as f32 * alpha) as u8,
        (color1[1] as f32 * inv_alpha + color2[1] as f32 * alpha) as u8,
        (color1[2] as f32 * inv_alpha + color2[2] as f32 * alpha) as u8,
        min(255, color1[3] + color2[3]), // Ensure alpha doesn't exceed 255
    ]
}




// }