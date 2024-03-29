use bevy::prelude::*;
use bevy_rapier2d::parry::utils::center;
use planet::{
    room::Room,
    roooms,
    triangulation::{self, find_mst_indexes},
    types::Coord,
};
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
                let d = dimension as u32;
                let s = ui_state.scale;

                if let Some(roooms) = &planet_data.roooms {
                    roooms.rooms.iter().for_each(|r| {
                        let c = r.center.into_world_normalized_vec2(&d) * s;
                        gizmos.circle_2d(c, 1., Color::RED);
                    })
                }

                if let Some(roooms) = &planet_data.roooms {
                    if let Some(mst) = &roooms.mst {
                        for i in mst {
                            let a = roooms.rooms[i.0].center.into_world_normalized_vec2(&d) * s;
                            let b = roooms.rooms[i.1].center.into_world_normalized_vec2(&d) * s;
                            gizmos.line_2d(a, b, Color::PINK);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GizmoOptions {
    pub draw_gizmos: bool,
    pub draw_centers: bool,
    pub draw_triangulation: bool,
    pub draw_mst: bool,
}

// fn doubler<T: Clone>(input: Vec<T>) -> Option<Vec<(T, T)>> {
//     if input.len() < 2 {
//         return None;
//     }

//     let tuples: Vec<(T, T)> = input
//         .windows(2)
//         .map(|window| (window[0].clone(), window[1].clone()))
//         .collect();

//     Some(tuples)
// }
