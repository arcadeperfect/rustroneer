use bevy::render::{color::Color, render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::Image};
use image::{ImageBuffer, Rgba};
use planet::{room::Room, tile_map::{Tile, TileMap}};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub const BLACK: [u8; 4] = [0, 0, 0, 255];
pub const WHITE: [u8; 4] = [255, 255, 255, 255];
pub const RED: [u8; 4] = [255, 0, 0, 255];
pub const GREEN: [u8; 4] = [0, 255, 0, 255];
pub const BLUE: [u8; 4] = [0, 0, 255, 255];

pub fn imagebuffer_to_bevy_image(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Image {
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

pub fn umap_to_bevy_image(map: &Vec<Vec<u8>>) -> Image {
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

pub fn fmap_to_bevy_image(map: &Vec<Vec<f32>>) -> Image {
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

pub fn room_vec_to_bevy_image(room_vec: &Vec<Room>, res: usize) -> Image {
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
            data[index] = c[0]; // R
            data[index + 1] = c[1]; // G
            data[index + 2] = c[2]; // B
            data[index + 3] = 255; // A (opacity)
        }

        let c_x = room.center.y;
        let c_y = room.center.x;
        let inverted = res - c_y - 1;
        let index = (inverted * res + c_x) * 4;
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

pub fn tile_map_to_bevy_image(map: &TileMap) -> Image {
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
                    Tile::RoomCenter(id) => random_room_color_accent((id) as u64),
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
