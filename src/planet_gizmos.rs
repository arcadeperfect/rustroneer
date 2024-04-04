use crate::{
    bevy_planet::lib::BevyPlanet, ui_state::UiState,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
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
        if let Some(planet_data) = &bevy_planet.planet_data
        {
            let dimension = planet_data.get_dimension();
            let dimension = dimension as u32;
            let scale = ui_state.scale;

            if let Some(roooms) = &planet_data.roooms {
                let o = Vec2::new(-1.7, 0.);

                if ui_state.gizmo_options.draw_mst {
                    // draw mst
                    if let Some(mst) = &roooms.mst {
                        for i in mst {
                            let a = ((roooms.rooms[i.0]
                                .center
                                .into_world_normalized_vec2(
                                    &dimension,
                                ))
                                + o)
                                * scale;
                            let b = ((roooms.rooms[i.1]
                                .center
                                .into_world_normalized_vec2(
                                    &dimension,
                                ))
                                + o)
                                * scale;
                            gizmos.line_2d(
                                a,
                                b,
                                Color::PINK,
                            );
                        }
                    }
                }
                if ui_state.gizmo_options.draw_centers {
                    // draw room centers
                    roooms.rooms.iter().for_each(|r| {
                        let c = (r
                            .center
                            .into_world_normalized_vec2(
                                &dimension,
                            )
                            + o)
                            * scale;
                        gizmos.circle_2d(
                            c,
                            scale * 0.01,
                            Color::RED,
                        );
                    });
                }

                if ui_state.gizmo_options.draw_triangulation
                {
                    // draw room triangulation
                    if let Some(tr) =
                        roooms.get_triangulation_coords()
                    {
                        tr.iter().for_each(|t| {
                            let start = (t
                                .0
                                .into_world_normalized_vec2(
                                    &dimension,
                                ))
                                + o * scale;
                            let end = (t
                                .1
                                .into_world_normalized_vec2(
                                    &dimension,
                                ))
                                + o * scale;

                            gizmos.line_2d(
                            (t.0.into_world_normalized_vec2(
                                &dimension,
                            ) + o) * scale,
                            (t.1.into_world_normalized_vec2(
                                &dimension,
                            ) + o) * scale,
                            Color::RED,
                        )
                        })
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
    pub offset: bool,
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
