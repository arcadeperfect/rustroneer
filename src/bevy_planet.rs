use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils::tracing,
};
use rand::prelude::*;
// use bevy_egui::egui::epaint::image;
use bevy_rapier2d::prelude::*;
use planet::{
    room::Room,
    tile_map::{Tile, TileMap},
    types::{FractalNoiseOptions, PlanetData},
    PlanetBuilder, PlanetOptions,
};

use crate::{
    constants::*,
    line::{LineList, LineMaterial},
    ui::UiChangedEvent,
    ui_state::{SelectedOption, UiState},
};

use image::{ImageBuffer, Rgba};

pub struct PlanetPlugin;

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

        // dbg!(&options);

        // let fractal_options = FractalNoiseOptions{
        //     frequency: state.noise1.frequency as f64,
        //     lacunarity: state.noise1.lacunarity as f64,
        //     octaves: state.noise1.octaves as usize,
        //     persistence: state.noise1.persistence as f64
        // };

        // if(state.noise.len() == 0){
        //     return;
        // }

        // let fractal_options_1 = state.noise[0].clone();

        // let fractal_options_vec = vec![
        //     &fractal_options_1
        // ];

        // let fractal_options = state.noise;

        // println!("fractal options: {:?}", state.noise.len());

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
    planet_query: Query<(Entity, &BevyPlanet, &NeedsColliderUpdate)>,
    collider_query: Query<Entity, With<PlanetColliderTag>>,
) {
    for (planet_entity, planet, _needs_update) in planet_query.iter() {
        if let Some(planet) = planet.planet_data.as_ref() {
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
    let scale = state.scale;

    for (planet_entity, bevy_planet) in planet_query.iter() {
        if let Some(planet) = bevy_planet.planet_data.as_ref() {
            if let Some(main_map) = &planet.planet_map.main {
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
            }

            commands
                .entity(planet_entity)
                .remove::<NeedsTextureUpdate>();
        }
    }
}

// fn update_planet_texture_transform(
//     state: Res<UiState>,
//     mut query: Query<&mut Transform, With<TexturePlanetRootTag>>,
//     mut events: EventReader<UiChangedEvent>,
// ) {
//     let ui_event = events.read();
//     // for _event in ui_event {
//     //     if let Ok(mut transform) = query.get_single_mut() {
//     //         transform.scale = Vec3::new(1., state.scale, state.scale);
//     //     }
//     // }
// }

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
        let mut vis = vis_query.single_mut();

        match state.show_texture {
            true => *vis = Visibility::Visible,
            false => *vis = Visibility::Hidden,
        }

        if let Some(planet) = bevy_planet.planet_data.as_ref() {
            if let Some(texture_plane_entity) = texture_planet_entity_resource.entity {
                if let Ok(mut material_handle) = query.get_mut(texture_plane_entity) {
                    let new_image = match state.bitmap_dislpay {
                        SelectedOption::PlanetRaw => match &planet.planet_map.main {
                            Some(main_map) => umap_to_bevy_image(main_map),
                            None => continue,
                        },
                        SelectedOption::PlanetProcessed => match &planet.image {
                            Some(image) => imagebuffer_to_bevy_image(image),
                            None => continue,
                        },
                        SelectedOption::Altitude => match &planet.planet_map.altitude {
                            Some(altitude) => fmap_to_bevy_image(altitude),
                            None => continue,
                        },
                        SelectedOption::Depth => match &planet.planet_map.depth {
                            Some(depth) => fmap_to_bevy_image(depth),
                            None => continue,
                        },
                        SelectedOption::RoomsRaw => match &planet.planet_map.rooms_raw {
                            Some(rooms) => umap_to_bevy_image(rooms),
                            None => continue,
                        },
                        // SelectedOption::RoomsDebug => match &planet.tile_map{
                        //     // Some(rooms) => imagebuffer_to_bevy_image(rooms_debug),
                        //     Some(t) => tile_map_to_bevy_image(t),
                        //     None => continue,
                        // }
                        // _ => continue,
                        SelectedOption::RoomsDebug => match &planet.rooms {
                            // Some(rooms) => imagebuffer_to_bevy_image(rooms_debug),
                            Some(t) => room_vec_to_bevy_image(t, planet.get_dimension().unwrap()),
                            None => continue,
                        },
                        _ => continue,
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

// fn update_planet_texture(
//     state: ResMut<UiState>,
//     texture_planet_entity_resource: Res<TexturePlaneEntityResource>,
//     planet_query: Query<(Entity, &BevyPlanet, &NeedsTextureUpdate), With<Name>>,
//     mut commands: Commands,
//     mut images: ResMut<Assets<Image>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut query: Query<&mut Handle<StandardMaterial>>,
//     mut vis_query: Query<&mut Visibility, With<TexturePlanetRootTag>>,
// ) {
//     for (planet_entity, bevy_planet, _) in planet_query.iter() {
//         let mut vis = vis_query.single_mut();

//         match state.show_texture {
//             true => {
//                 *vis = Visibility::Visible;
//             }
//             false => {
//                 *vis = Visibility::Hidden;
//             }
//         }

//         if let Some(planet) = bevy_planet.planet_data.as_ref() {
//             // if let Some(main_map) = &planet.planet_map.main {
//             if let Some(texture_plane_entity) = texture_planet_entity_resource.entity {
//                 if let Ok(mut material_handle) = query.get_mut(texture_plane_entity) {
//                     // let new_image = imagebuffer_to_bevy_image(&planet.image);

//                     if let Some(altitude) = &planet.planet_map.altitude {
//                         let new_image = fmap_to_bevy_image(&altitude);

//                         // let new_image = fmap_to_bevy_image(planet.planet_map.altitude.unwrap().as_ref());
//                         // let new_image = planet_map_to_bevy_image(&main_map);
//                         let new_image_handle = images.add(new_image);

//                         // Update the material with the new image
//                         let new_material = StandardMaterial {
//                             base_color_texture: Some(new_image_handle),
//                             unlit: true,
//                             ..default()
//                         };

//                         // Update the material in the assets and apply it to the entity
//                         *material_handle = materials.add(new_material);
//                     }

//                 }
//             }

//             commands
//                 .entity(planet_entity)
//                 .remove::<NeedsTextureUpdate>();
//             // }
//         }
//     }
// }

fn imagebuffer_to_bevy_image(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Image {
    let width = buffer.width();
    let height = buffer.height();

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let dimension = TextureDimension::D2;

    let data = buffer.as_raw();

    let format = TextureFormat::Rgba8UnormSrgb;
    let asset_usage = RenderAssetUsages::RENDER_WORLD;

    Image::new(size, dimension, data.clone(), format, asset_usage)
}

fn umap_to_bevy_image(map: &Vec<Vec<u8>>) -> Image {
    let width = map.len() as u32;
    let height = map[0].len() as u32;

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let dimension = TextureDimension::D2;

    let data: Vec<u8> = map
        .iter()
        .rev()
        .flat_map(|row| {
            row.iter().flat_map(|&v| {
                let v = (v * 100) as u8; // Convert u16 to u8, might need different conversion based on your data
                vec![v, v, v, 10u8] // R, G, B, A
            })
        })
        .collect();

    let format = TextureFormat::Rgba8UnormSrgb;
    let asset_usage = RenderAssetUsages::RENDER_WORLD;

    Image::new(size, dimension, data, format, asset_usage)
}

fn fmap_to_bevy_image(map: &Vec<Vec<f32>>) -> Image {
    let width = map.len() as u32;
    let height = map[0].len() as u32;

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let dimension = TextureDimension::D2;

    let data: Vec<u8> = map
        .iter()
        .flat_map(|row| {
            row.iter().flat_map(|&v| {
                let v = (v * 255.0) as u8; // Convert u16 to u8, might need different conversion based on your data
                vec![v, v, v, 10u8] // R, G, B, A
            })
        })
        .collect();

    let format = TextureFormat::Rgba8UnormSrgb;
    let asset_usage = RenderAssetUsages::RENDER_WORLD;

    Image::new(size, dimension, data, format, asset_usage)
}

fn room_vec_to_bevy_image(room_vec: &Vec<Room>, res: usize) -> Image {
    let size = Extent3d {
        width: res as u32,
        height: res as u32,
        depth_or_array_layers: 1,
    };

    let dimension = TextureDimension::D2;

    let mut data: Vec<u8> = vec![0; res * res * 4];

    for room in room_vec {
        for tile in &room.tiles {
            let x = tile.y as usize;
            let y = res - tile.x as usize - 1;
            let index = (y * res + x) * 4;
            let c = random_room_color(room.id as u64);
            data[index] = c[0]; // R
            data[index + 1] = c[1]; // G
            data[index + 2] = c[2]; // B
            data[index + 3] = 255; // A (opacity)
        }

        for tile in &room.edge_tile_indexes {
            let x = room.tiles[*tile].y as usize;
            let y = res - room.tiles[*tile].x as usize - 1;

            let index = (y * res + x) * 4;
            let c = random_room_color_accent(room.id as u64);
            // let c = WHITE;
            data[index] = c[0]; // R
            data[index + 1] = c[1]; // G
            data[index + 2] = c[2]; // B
            data[index + 3] = 255; // A (opacity)
        }

        let c_x = room.center.y;
        let c_y = room.center.x;

        let inverted = res - c_y - 1;

        println!("{} {} {}", res, c_y, inverted);

        let index = (inverted * res + c_x) * 4;
        // let c_c = random_room_center_color(room.id as u64);
        let c_c = GREEN;
        data[index] = c_c[0]; // R
        data[index + 1] = c_c[1]; // G
        data[index + 2] = c_c[2]; // B
        data[index + 3] = 255; // A (opacity)
    }

    let format = TextureFormat::Rgba8UnormSrgb;
    let asset_usage = RenderAssetUsages::RENDER_WORLD;

    Image::new(size, dimension, data, format, asset_usage)
}

fn tile_map_to_bevy_image(map: &TileMap) -> Image {
    let width = map[0].len() as u32; // Assuming all rows have the same length
    let height = map.len() as u32;

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let dimension = TextureDimension::D2;

    let data: Vec<u8> = map
        .iter()
        .rev()
        .flat_map(|row| {
            row.iter().flat_map(|&tile| {
                match tile {
                    Tile::Space => BLACK,
                    Tile::Wall => WHITE, // White for wall
                    Tile::Room(id) => random_room_color((id).unwrap() as u64),
                    // Tile::RoomCenter(id) => random_room_center_color((id) as u64),
                    Tile::RoomCenter(id) => BLACK,
                    _ => GREEN, // Add more cases as needed
                }
            })
        })
        .collect();

    let format = TextureFormat::Rgba8UnormSrgb;
    let asset_usage = RenderAssetUsages::RENDER_WORLD;

    Image::new(size, dimension, data, format, asset_usage)
}

fn random_room_color(id: u64) -> [u8; 4] {
    let mut rng = StdRng::seed_from_u64(id);
    let random_float: f32 = rng.gen();

    let clr = Color::Lcha {
        lightness: 0.5,
        chroma: 0.5,
        hue: random_float * 360.0,
        alpha: 1.0,
    };

    let [r, g, b, a] = clr.as_rgba_f32();
    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ]
}

fn random_room_color_accent(id: u64) -> [u8; 4] {
    let mut rng = StdRng::seed_from_u64(id);
    let random_float: f32 = rng.gen();

    let clr = Color::Lcha {
        lightness: 0.8,
        chroma: 0.8,
        hue: random_float * 360.0,
        alpha: 1.0,
    };

    let [r, g, b, a] = clr.as_rgba_f32();
    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ]
}
