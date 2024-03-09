use bevy::prelude::*;
use bevy_planet::PlanetBuilderResource;
use bevy_rapier2d::prelude::*;
use camera::MyCameraPlugin;
use physics::PhysicsPlugin;
use planet::{PlanetData, PlanetOptions};
use player::MyPlayerPlugin;
use ui::{PlanetUiPlugin, UiChangedEvent};
use crate::types::UiState;

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
        .add_plugins(RapierDebugRenderPlugin {
            style: DebugRenderStyle {
                rigid_body_axes_length: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins((
            PlanetUiPlugin,
            MyPlayerPlugin,
            PhysicsPlugin,
            MyCameraPlugin,
        ))
        .run();
}
