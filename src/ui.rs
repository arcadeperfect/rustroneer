use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::{Event, EventWriter},
        system::{ResMut, Resource},
    },
};
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use strum::IntoEnumIterator; // Import necessary traits

use crate::{player::{PlayerEvent, PlayerEventType}, ui_state::{self, BitmapDisplay, UiState}};


pub struct PlanetUiPlugin;

impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        let ui_state: UiState = UiState::load().unwrap_or_default();

        app.add_plugins(WorldInspectorPlugin::new())
            // .add_plugins(bevy_planet::PlanetPlugin)
            .insert_resource(ui_state)
            .add_event::<RegeneratePlanetEvent>()
            .add_systems(Startup, init_planet_system)
            .add_systems(Update, ui_system)
            .add_event::<RegeneratePlanetEvent>()
            .add_event::<GeneralUpdateEvent>()
            .init_resource::<OccupiedScreenSpace>();
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
pub struct GeneralUpdateEvent {
    pub ui_state: UiState,
}

fn init_planet_system(state: ResMut<UiState>, mut event_writer: EventWriter<RegeneratePlanetEvent>) {
    event_writer.send(RegeneratePlanetEvent {
        ui_state: state.clone(),
    });
}

fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut ui_event_writer: EventWriter<RegeneratePlanetEvent>,
    mut player_event_writer: EventWriter<crate::player::PlayerEvent>,
    mut general_update_event_writer: EventWriter<GeneralUpdateEvent>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {

    let mut generation_settings_changed = false;
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

            generation_settings_changed |= ui
                .add(egui::Slider::new(&mut state.radius, 0.0..=1.0).text("circle radius"))
                .changed();

            generation_settings_changed |= ui
                .add(
                    egui::Slider::new(&mut state.crust_thickness, 0.0..=1.0)
                        .text("crust thickness"),
                )
                .changed();

            generation_settings_changed |= ui
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
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 2 Parameters", |ui| {
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 3 Parameters", |ui| {
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                    generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise mask parameters", |ui| {
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise_mask_options.mask_frequency, 0.0..=3.0)
                            .text("mask frequency"),
                    )
                    .changed();
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise_mask_options.mask_z, 0.0..=20.0)
                            .text("mask z"),
                    )
                    .changed();
            });

            ui.collapsing("Displacement parameters", |ui| {
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.displacement_scale, 0.0..=10.0)
                            .text("displacement scale"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.displacement_frequency, 0.0..=10.0)
                            .text("displacement frequency"),
                    )
                    .changed();
            });

            ui.collapsing("Global noise parameters", |ui| {
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.global_noise_options.amplitude, 0.0..=2.0)
                            .text("global amplitude"),
                    )
                    .changed();
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.global_noise_options.frequency, 0.0..=2.0)
                            .text("global frequency"),
                    )
                    .changed();
                ui.horizontal(|ui| {
                    generation_settings_changed |= ui
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
                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.init_weight, 0.0..=1.)
                            .text("c.a. init noise weight"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.iterations, 0..=150)
                            .text("c.a. iterations"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.threshold, 0..=16)
                            .text("c.a. threshold"),
                    )
                    .changed();

                generation_settings_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.search_radius, 0..=15)
                            .text("c.a. search radius"),
                    )
                    .changed();

                ui.horizontal(|ui| {
                    generation_settings_changed |= ui
                        .add(egui::DragValue::new(&mut state.ca_options.seed))
                        .changed();
                    ui.label("seed");
                });

                generation_settings_changed |= ui.checkbox(&mut state.invert_ca, "Invert").changed();

                ui.collapsing("Mask", |ui| {
                    generation_settings_changed |= ui
                        .add(
                            egui::Slider::new(&mut state.ca_options.mask_options.mult, 0.0..=20.0)
                                .text("mult"),
                        )
                        .changed();
                    generation_settings_changed |= ui
                        .add(
                            egui::Slider::new(&mut state.ca_options.mask_options.lift, 0.0..=20.0)
                                .text("lift"),
                        )
                        .changed();
                });
            });

            ui.collapsing("Post Processing", |ui| {
                generation_settings_changed |= ui
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

            generation_settings_changed |= ui.checkbox(&mut state.rooms, "Rooms").changed();



            ui.collapsing("Player", |ui| {
                player_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.player_move_force, 0.0..=60.).text("move force"))
                    .changed();
                player_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.player_jetpack_force, 0.0..=60.).text("jetpack force"))
                    .changed();
                player_settings_changed |= ui
                    .add(egui::Slider::new(&mut state.player_rotate_force, -10.0..=10.).text("rotate force"))
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

            // ui.collapsing("Player", |ui| {
            //     ui_changed |= ui
            //         .add(egui::Slider::new(&mut state.player_move_force, 0.0..=60.).text("move force"))
            //         .changed();
            //     ui_changed |= ui
            //         .add(egui::Slider::new(&mut state.player_jetpack_force, 0.0..=60.).text("jetpack force"))
            //         .changed();
            //     ui_changed |= ui
            //         .add(egui::Slider::new(&mut state.player_rotate_force, -10.0..=10.).text("rotate force"))
            //         .changed();
            //     if ui.button("Reset").clicked() {
            //         player_event_writer.send(crate::player::PlayerEvent{event_type: crate::player::PlayerEventType::Respawn});
            //     }
            // });

            // ui.collapsing("Camera", |ui| {
            //     ui_changed |= ui
            //         .add(egui::Checkbox::new(&mut state.game_camera, "game_cam"))
            //         .changed();

            //         ui_changed |= ui
            //         .add(egui::Slider::new(&mut state.game_camera_zoom, -5.0..=50.).text("zoom"))
            //         .changed();
            // });

            if generation_settings_changed {
                state.save().ok();
                ui_event_writer.send(RegeneratePlanetEvent {
                    ui_state: state.clone(),
                });
            }

            if player_settings_changed {
                player_event_writer.send(PlayerEvent{
                    event_type: PlayerEventType::RefreshPlayer,
                    ui_state: state.clone()});
            }

            if camera_settings_changed {
                player_event_writer.send(PlayerEvent{
                    event_type: PlayerEventType::RefreshCam,
                    ui_state: state.clone()});
            }

            if general_changed{
                general_update_event_writer.send(GeneralUpdateEvent{
                    ui_state: state.clone()
                });
            }

        })
        .response
        .rect
        .width();
}
