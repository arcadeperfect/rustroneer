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
        // .add_systems(Update, rebuild_planet)
        .run();
}

// fn temp_function(){

// }

// fn rebuild_planet(
//     mut commands: Commands,
//     mut builder_resource: ResMut<PlanetBuilderResource>,
//     mut events: EventReader<SliderChangedEvent>
// ) {


//     // let t = UiState::default();
//     // println!("{:?}", t);

//     if !events.is_empty() {
//         let v = events.read();
//         for i in v{
//             // println!("{:?}", i.ui_state);

//             let options = PlanetOptions::from(i.ui_state.clone());

//             let new_planet: Planet = builder_resource.builder.build(options).unwrap();

//         }
//         events.clear();
//     }
// }


// impl From<UiState> for PlanetOptions{
//     fn from(ui_state: UiState) -> Self {
//         Self {
//             seed: 0,
//             min_room_size: 20,

//             frequency: ui_state.frequency,
//             amplitude: ui_state.amplitude,
//             radius: ui_state.radius,
//             resolution: ui_state.resolution,
//             thresh: ui_state.thresh,
//             iterations: ui_state.iterations,
//             weight: ui_state.weight,
//             blur: ui_state.blur,

//         }
//     }
// }