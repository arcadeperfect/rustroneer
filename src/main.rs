use bevy::{
    log::tracing_subscriber::{self, EnvFilter},
    prelude::*,
    window::WindowMode,
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use camera::MyCameraPlugin;
use physics::PhysicsPlugin;
use player::MyPlayerPlugin;
use ui::PlanetUiPlugin;
use planet_gizmos::PlanetGizmosPlugin;

mod bevy_planet;
mod camera;
mod line;
mod physics;
mod player;
mod traits;
mod types;
mod ui;
mod ui_state;
mod planet_gizmos;
mod vector_shapes;

#[derive(Component)]
pub struct PlayerTag;

fn main() {
    match dotenv::dotenv() {
        Ok(_) => {
            println!("loaded env")
        }
        Err(e) => {
            println!("error loading env: {}", e)
        }
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen,
                        // mode: WindowMode::SizedFullscreen,
                        // mode::WindowMode::
                        // resolution: (1500., 800.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(bevy_planet::lib::PlanetPlugin)
        .add_plugins((
            PlanetUiPlugin,
            MyCameraPlugin,
            MyPlayerPlugin,
            PhysicsPlugin,
        ))
        .add_plugins(TweeningPlugin)
        .add_plugins(PlanetGizmosPlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .run();
}
