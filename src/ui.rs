
use bevy::{
    app::{App, Plugin, Startup, Update}, core_pipeline::core_3d::Camera3d, ecs::{
        event::{Event, EventReader, EventWriter}, query::With, system::{Query, Res, ResMut, Resource}
    }, input::{mouse::{MouseButton, MouseButtonInput}, ButtonInput}, math::primitives::Plane3d, render::camera::Camera, transform::components::{GlobalTransform, Transform}, window::CursorMoved
};
use bevy_egui::{
    egui, EguiContext, EguiContexts, EguiPlugin
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use glam::{Vec2, Vec3};
use strum::IntoEnumIterator; // Import necessary traits

use crate::{bevy_planet::lib::PlanetRootTag, player::{PlayerEvent, PlayerEventType}, ui_state::{self, BitmapDisplay, UiState}};


pub struct PlanetUiPlugin;

impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        let ui_state: UiState = UiState::load().unwrap_or_default();

        app.add_plugins(WorldInspectorPlugin::new())
            .insert_resource(ui_state)
            .add_event::<RegeneratePlanetEvent>()
            .add_event::<ModifyMeshEvent>()
            .add_systems(Startup, init_planet_system)
            .add_systems(Update, ui_system)
            .add_event::<RegeneratePlanetEvent>()
            .add_event::<GeneralUpdateEvent>()
            .add_event::<MouseClickWorldEvent>()
            .init_resource::<OccupiedScreenSpace>()
            .add_systems(Update, mouse_click_world);
    }
}

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    // top: f32,
    // right: f32,
    // bottom: f32,
}


#[derive(Event, Debug)]
pub struct RegeneratePlanetEvent {
    pub ui_state: UiState,
}

#[derive(Event, Debug)]
pub struct ModifyMeshEvent {
    pub ui_state: UiState,
    pub moust_position: Vec2,
    pub modification: MeshModification,
}

#[derive(Debug)]
pub enum MeshModification{
    Add(u32),
    Remove(u32),
}

#[derive(Event, Debug)]
pub struct GeneralUpdateEvent {
    pub ui_state: UiState,
}

fn init_planet_system(state: ResMut<UiState>, mut event_writer: EventWriter<RegeneratePlanetEvent>) {
    event_writer.send(RegeneratePlanetEvent {
        ui_state: state.clone(),
    });
}


#[derive(Event, Debug)]
pub struct MouseClickWorldEvent {
    pub pos: Vec3,
    pub button: MouseButton,
}




fn mouse_click_world(
    clicked: Res<ButtonInput<MouseButton>>,
    mut cursor_position: EventReader<CursorMoved>, 
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_plane: Query<&GlobalTransform, With<PlanetRootTag>>,
    occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut click_event_writer: EventWriter<MouseClickWorldEvent>,
) {
    let (camera, camera_transform) = q_camera.single();
    let ground_transform = q_plane.single();


    for event in cursor_position.read() {
        // println!("{:?}", event.position);
        let plane_origin = ground_transform.translation();
        let plane = Plane3d::new(Vec3::new(0., 0., -1.));
    
        // Ask Bevy to give us a ray pointing from the viewport (screen) into the world
        let offset_vec = Vec2::new(event.position.x - occupied_screen_space.left, event.position.y);
        let Some(ray) = camera.viewport_to_world(camera_transform, offset_vec) else {
            println!("Could not convert cursor position to world");
            return;
        };
    
        // do a ray-plane intersection test, giving us the distance to the ground
        let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
            // If the ray does not intersect the ground
            // (the camera is not looking towards the ground), we can't do anything

            println!("Not looking at the ground");
            return;
        };

        // use the distance to compute the actual point on the ground in world-space
        let global_cursor = ray.get_point(distance);
        
        // println!("{}", global_cursor);

        if clicked.pressed(MouseButton::Left) {
            click_event_writer.send(
                MouseClickWorldEvent {
                    pos: global_cursor,
                    button: MouseButton::Left
            });
        }

        if clicked.pressed(MouseButton::Right) {
            click_event_writer.send(
                MouseClickWorldEvent {
                    pos: global_cursor,
                    button: MouseButton::Right
            });
        }
    }
}


fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut ui_event_writer: EventWriter<RegeneratePlanetEvent>,
    mut player_event_writer: EventWriter<crate::player::PlayerEvent>,
    mut mesh_event_writer: EventWriter<ModifyMeshEvent>,
    mut general_update_event_writer: EventWriter<GeneralUpdateEvent>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {

    let mut planet_gen_settings_changed = false;
    let mut player_settings_changed = false;
    let mut camera_settings_changed = false;
    let mut general_changed = false;
    

    let smaller_space = 10.;
    let larger_space = 5.;

    occupied_screen_space.left = egui::SidePanel::left("Hello")
        .default_width(500.)
        .show(contexts.ctx_mut(), |ui| {
            ui.style_mut().spacing.slider_width = 300.0;

            ui.add_space(smaller_space);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;

            ui.label(
                egui::RichText::new("Initial")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(larger_space);

            planet_gen_settings_changed |= ui
                .add(egui::Slider::new(&mut state.radius, 0.0..=1.0).text("circle radius"))
                .changed();

            planet_gen_settings_changed |= ui
                .add(
                    egui::Slider::new(&mut state.crust_thickness, 0.0..=1.0)
                        .text("crust thickness"),
                )
                .changed();

            planet_gen_settings_changed |= ui
                .add(egui::Slider::new(&mut state.resolution, 10..=1600).text("resolution"))
                .changed();

            ui.add_space(smaller_space);
            let heading_style = egui::TextStyle::Heading;
            ui.label(
                egui::RichText::new("Noise")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(larger_space);

            let mut _noise_parameters_open = true;
            ui.collapsing("Noise 1 Parameters", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 2 Parameters", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 3 Parameters", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                    planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise mask parameters", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise_mask_options.mask_frequency, 0.0..=3.0)
                            .text("mask frequency"),
                    )
                    .changed();
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise_mask_options.mask_z, 0.0..=20.0)
                            .text("mask z"),
                    )
                    .changed();
            });

            ui.collapsing("Displacement parameters", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.displacement_scale, 0.0..=10.0)
                            .text("displacement scale"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.displacement_frequency, 0.0..=10.0)
                            .text("displacement frequency"),
                    )
                    .changed();
            });

            ui.collapsing("Global noise parameters", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.global_noise_options.amplitude, 0.0..=2.0)
                            .text("global amplitude"),
                    )
                    .changed();
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.global_noise_options.frequency, 0.0..=2.0)
                            .text("global frequency"),
                    )
                    .changed();
                ui.horizontal(|ui| {
                    planet_gen_settings_changed |= ui
                        .add(egui::DragValue::new(&mut state.global_noise_options.z))
                        .changed();
                    ui.label("Z:");
                });
            });

            ui.add_space(smaller_space);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Cellular Automata")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(larger_space);

            ui.collapsing("Cellular Automata", |ui| {
                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.init_weight, 0.0..=1.)
                            .text("c.a. init noise weight"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.iterations, 0..=150)
                            .text("c.a. iterations"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.threshold, 0..=16)
                            .text("c.a. threshold"),
                    )
                    .changed();

                planet_gen_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.search_radius, 0..=15)
                            .text("c.a. search radius"),
                    )
                    .changed();

                ui.horizontal(|ui| {
                    planet_gen_settings_changed |= ui
                        .add(egui::DragValue::new(&mut state.ca_options.seed))
                        .changed();
                    ui.label("seed");
                });

                planet_gen_settings_changed |= ui.checkbox(&mut state.invert_ca, "Invert").changed();

                ui.collapsing("Mask", |ui| {
                    planet_gen_settings_changed |= ui
                        .add(
                            egui::Slider::new(&mut state.ca_options.mask_options.mult, 0.0..=20.0)
                                .text("mult"),
                        )
                        .changed();
                    planet_gen_settings_changed |= ui
                        .add(
                            egui::Slider::new(&mut state.ca_options.mask_options.lift, 0.0..=20.0)
                                .text("lift"),
                        )
                        .changed();
                });
            });

            ui.collapsing("Post Processing", |ui| {
                planet_gen_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.blur, 0.0..=8.).text("post blur"))
                    .changed();
            });

            ui.add_space(smaller_space);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Display")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(larger_space);

            for variant in BitmapDisplay::iter() {
                general_changed |= ui
                    .radio_value(
                        &mut state.bitmap_dislpay,
                        variant.clone(),
                        format!("{:?}", variant),
                    )
                    .changed();
            }

            ui.add_space(smaller_space);

            general_changed |= ui
                .checkbox(&mut state.show_texture, "Show texture")
                .changed();

                general_changed |= ui
                .checkbox(&mut state.show_vectors, "Show Vectors")
                .changed();

                general_changed |= ui.checkbox(&mut state.show_debug, "Show Debug").changed();

            ui.add_space(10.0);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Entity")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );

            general_changed |= ui
                .add(egui::Slider::new(&mut state.scale, 5.0..=400.).text("scale"))
                .changed();

            ui.add_space(larger_space);

            general_changed |= ui
                .checkbox(&mut state.gizmo_options.draw_gizmos, "Draw Gizmos")
                .changed();

            planet_gen_settings_changed |= ui.checkbox(&mut state.rooms, "Rooms").changed();



            ui.collapsing("Player", |ui| {
                player_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.player_move_force, 0.0..=60.).text("move force"))
                    .changed();
                player_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.player_jetpack_force, 0.0..=60.).text("jetpack force"))
                    .changed();
                player_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.player_rotate_force, 0.0..=1.).text("rotate force"))
                    .changed();
                if ui.button("Reset").clicked() {
                    player_event_writer.send(crate::player::PlayerEvent{event_type: crate::player::PlayerEventType::Respawn, ui_state: state.clone()});
                }
            });

            ui.collapsing("Camera", |ui| {
                camera_settings_changed |= ui
                    .add(egui::Checkbox::new(&mut state.game_camera, "game_cam"))
                    .changed();

                camera_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.game_camera_zoom, -5.0..=50.).text("zoom"))
                    .changed();
            });

            if ui.button("Refresh Mesh").clicked() {
                println!("Refreshing Mesh clicked");
                mesh_event_writer.send(ModifyMeshEvent {
                    ui_state: state.clone(),
                    modification: MeshModification::Add(0),
                    moust_position: Vec2::new(0.0, 0.0),
                });
            }

            if planet_gen_settings_changed {
                state.save().ok();
                ui_event_writer.send(RegeneratePlanetEvent {
                    ui_state: state.clone(),
                });
            }

            if player_settings_changed {
                state.save().ok();
                player_event_writer.send(PlayerEvent{
                    event_type: PlayerEventType::RefreshPlayer,
                    ui_state: state.clone()});
            }

            if camera_settings_changed {
                state.save().ok();
                player_event_writer.send(PlayerEvent{
                    event_type: PlayerEventType::RefreshCam,
                    ui_state: state.clone()});
            }

            if general_changed{
                state.save().ok();
                general_update_event_writer.send(GeneralUpdateEvent{
                    ui_state: state.clone()
                });
            }

        })
        .response
        .rect
        .width();
}
