use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use camera::MyCameraPlugin;
use physics::PhysicsPlugin;

use player::MyPlayerPlugin;
use ui::PlanetUiPlugin;


mod bevy_planet;
mod camera;
mod line;
mod physics;
mod player;
mod traits;
mod ui;
mod types;

#[derive(Component)]
pub struct PlayerTag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(bevy_planet::PlanetPlugin)
        .add_plugins((
            PlanetUiPlugin,
            MyPlayerPlugin,
            PhysicsPlugin,
            MyCameraPlugin,
        ))
        .run();
}
