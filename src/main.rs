use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use camera::MyCameraPlugin;
use physics::PhysicsPlugin;
use player::MyPlayerPlugin;


mod player;
mod traits;
mod camera;
mod physics;
mod bevy_planet;




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
            MyPlayerPlugin,
            PhysicsPlugin, 
            MyCameraPlugin,
            bevy_planet::PlumbetPlugin
        ))
        .add_plugins(WorldInspectorPlugin::new())
        // .add_systems(Startup, (spawn_planet))
        .run();
}





// fn build_planet(){
//     let p = PlanetBuilder::new(PlanetOptions::default());
    
// }