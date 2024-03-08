use bevy::prelude::*;
use bevy_rapier2d::{dynamics::ExternalForce, plugin::RapierConfiguration};

use crate::traits::IntoVec2;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics);
        app.add_systems(Update, apply_gravity);
    }
}

fn setup_physics(mut rapier_conf: ResMut<RapierConfiguration>) {
    rapier_conf.gravity = Vec2::ZERO;
}

fn apply_gravity(mut query: Query<(&mut ExternalForce, &Transform)>) {
    let grav_scale = 5.;

    for (mut ef, t) in query.iter_mut() {
        ef.force = -t.translation.into_vec2().normalize() * grav_scale;
    }
}