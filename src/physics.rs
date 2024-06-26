use bevy::prelude::*;

use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_rapier2d::render::{
    DebugRenderContext, DebugRenderStyle,
    RapierDebugRenderPlugin,
};


use crate::ui::GeneralUpdateEvent;
use crate::ui_state;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics)
            // .add_systems(Update, apply_gravity)
            .add_plugins(RapierDebugRenderPlugin {
                style: DebugRenderStyle {
                    rigid_body_axes_length: 0.5,
                    ..Default::default()
                },
                enabled: true,
                ..Default::default()
            })
            .add_systems(Update, update_debug);
    }
}

fn setup_physics(
    mut rapier_conf: ResMut<RapierConfiguration>,
    mut rapier_debug_render_plugin: ResMut<
        DebugRenderContext,
    >,
) {
    rapier_conf.gravity = Vec2::ZERO;
    rapier_debug_render_plugin.enabled = false;
            
}

// fn apply_gravity(mut query: Query<(&mut ExternalForce, &Transform)>) {
//     let grav_scale = 5.;

//     for (mut ef, t) in query.iter_mut() {
//         ef.force = -t.translation.into_vec2().normalize() * grav_scale;
//     }
// }

fn update_debug(
    mut rapier_debug_render_plugin: ResMut<
        DebugRenderContext,
    >,
    mut events: EventReader<GeneralUpdateEvent>,
    ui_state: Res<ui_state::UiState>,
) {
    for event in events.read() {
        // println!("{:?}", event.ui_state.show_debug);
        rapier_debug_render_plugin.enabled =
            ui_state.show_debug;
        // rapier_debug_render_plugin.enabled = false;
    }
}
