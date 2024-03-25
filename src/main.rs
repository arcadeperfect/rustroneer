use bevy::{log::tracing_subscriber::{self, EnvFilter}, prelude::*, window::WindowMode};
use bevy_rapier2d::prelude::*;
use camera::MyCameraPlugin;
use physics::PhysicsPlugin;
use player::MyPlayerPlugin;
use ui::PlanetUiPlugin;
use bevy_tweening::*;

mod bevy_planet;
mod camera;
mod line;
mod physics;
mod player;
mod traits;
mod types;
mod ui;
mod constants;
mod ui_state;

#[derive(Component)]
pub struct PlayerTag;

fn main() {

    // dotenv::dotenv().ok();

    match dotenv::dotenv() {
        Ok(_) => {println!("loaded env")}
        Err(e) => {println!("error loading env: {}", e)}    
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
                        // resolution: (1500., 800.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(bevy_planet::PlanetPlugin)
        .add_plugins((
            PlanetUiPlugin,
            MyPlayerPlugin,
            PhysicsPlugin,
            MyCameraPlugin,
        ))
        .add_plugins(TweeningPlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .run();
}



// fn spawn(mut commands: Commands) {
//     // Create a single animation (tween) to move an entity.
//     let tween = Tween::new(
//         // Use a quadratic easing on both endpoints.
//         EaseFunction::QuadraticInOut,
//         // Animation time (one way only; for ping-pong it takes 2 seconds
//         // to come back to start).
//         Duration::from_secs(1),
//         // The lens gives the Animator access to the Transform component,
//         // to animate it. It also contains the start and end values associated
//         // with the animation ratios 0. and 1.
//         TransformPositionLens {
//             start: Vec3::ZERO,
//             end: Vec3::new(1., 2., -4.),
//         },
//     )
//     // Repeat twice (one per way)
//     .with_repeat_count(RepeatCount::Finite(2))
//     // After each iteration, reverse direction (ping-pong)
//     .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

//     commands.spawn((
//         // Spawn a Sprite entity to animate the position of.
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::RED,
//                 custom_size: Some(Vec2::new(size, size)),
//                 ..default()
//             },
//             ..default()
//         },
//         // Add an Animator component to control and execute the animation.
//         Animator::new(tween),
//     ));
// }
