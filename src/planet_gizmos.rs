use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{bevy_planet::lib::BevyPlanet, ui_state::UiState};

pub struct PlanetGizmosPlugin;

impl Plugin for PlanetGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_gizmos);
    }
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    ui_state: Res<UiState>,
    planet_query: Query<&BevyPlanet, With<Name>>,
) {
    if ui_state.gizmo_options.draw_gizmos == false {
        return;
    }

    for bevy_planet in planet_query.iter() {

        if let Some(planet_data) = &bevy_planet.planet_data {
            if let Some(dimension) = planet_data.get_dimension() {
                let s = ui_state.scale;
                if let Some(rooms) = &planet_data.rooms {
                    for room in rooms {
                        gizmos.circle_2d(
                            room.center.into_world_normalized_vec2(&(dimension as u32)) * s,
                            1.,
                            Color::RED,
                        );
                    }
                }
            }
        }

        
        // let r = bevy_planet.planet_data.as_ref().unwrap().get_dimension().unwrap();
        // let s = ui_state.scale;

        // if let Some(planet_data) = &bevy_planet.planet_data {
        //     if let Some(rooms) = &planet_data.rooms {
        //         for room in rooms {
        //             // println!("room center: {:?} - as vec: {:?}", room.center, room.center.into_vec2());
        //             gizmos.circle_2d(
        //                 room.center.into_world_normalized_vec2(&(r as u32)) * s, 
        //                 1., Color::RED);
        //         }
        //     }
        // }

        // let planet_data = bevy_planet.planet_data.as_ref().unwrap();
        // let rooms = planet_data.rooms.as_ref().unwrap();

        // for room in rooms {
        //     gizmos.circle_2d(room.center.into_vec2(), 1., Color::RED);
        // }

        // more gizmos will be added here
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GizmoOptions {
    pub draw_gizmos: bool,
    pub draw_centers: bool,
    pub draw_triangulation: bool,
    pub draw_mst: bool,
}
