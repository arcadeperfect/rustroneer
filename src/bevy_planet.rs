use bevy::{ecs::bundle, prelude::*};
use bevy_rapier2d::prelude::*;
use planet::{Planet, PlanetBuilder, PlanetOptions};

pub struct PlumbetPlugin;

impl Plugin for PlumbetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planet);
    }
}

fn spawn_planet(mut commands: Commands) {
    let planet = get_planet();
    let colliders = get_colliders(planet.contours);

    let parent = commands.spawn(SpatialBundle::default()).insert(Name::new("Parent")).id();
    let child1 = commands
        .spawn(colliders[0].clone())
        .insert(TransformBundle::from(Transform::default()))
        .insert(Name::new("Child1"))
        .id();
    let child2 = commands
        .spawn(colliders[1].clone())
        .insert(TransformBundle::from(Transform::default()))
        .insert(Name::new("Child2"))
        .id();

    commands.entity(parent).push_children(&[child1, child2]);
}

fn get_colliders(vecs: Vec<Vec<Vec2>>) -> Vec<Collider> {
    let mut colliders = Vec::new();
    for vec in vecs {
        colliders.push(Collider::polyline(vec, None));
    }
    return colliders;
}

fn get_planet() -> Planet {
    let PlanetOptions = PlanetOptions::default();
    return PlanetBuilder::new(PlanetOptions);
}
