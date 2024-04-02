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

use crate::ui_state::{BitmapDisplay, UiState};

pub struct PlanetUiPlugin;

impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        let ui_state: UiState = UiState::load().unwrap_or_default();

        app.add_plugins(WorldInspectorPlugin::new())
            // .add_plugins(bevy_planet::PlanetPlugin)
            .insert_resource(ui_state)
            .add_event::<UiChangedEvent>()
            .add_systems(Startup, init_planet_system)
            .add_systems(Update, ui_system)
            .add_event::<UiChangedEvent>()
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
pub struct UiChangedEvent {
    pub ui_state: UiState,
}

fn init_planet_system(state: ResMut<UiState>, mut event_writer: EventWriter<UiChangedEvent>) {
    event_writer.send(UiChangedEvent {
        ui_state: state.clone(),
    });
}

fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut event_writer: EventWriter<UiChangedEvent>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let mut ui_changed = false;

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

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.radius, 0.0..=1.0).text("circle radius"))
                .changed();

            ui_changed |= ui
                .add(
                    egui::Slider::new(&mut state.crust_thickness, 0.0..=1.0)
                        .text("crust thickness"),
                )
                .changed();

            ui_changed |= ui
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
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[0].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 2 Parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[1].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 3 Parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();

                    ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.fractal_noises[2].offset, -1.5..=1.5)
                            .text("noise offset"),
                    )
                    .changed();
            });

            ui.collapsing("Noise mask parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise_mask_options.mask_frequency, 0.0..=3.0)
                            .text("mask frequency"),
                    )
                    .changed();
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise_mask_options.mask_z, 0.0..=20.0)
                            .text("mask z"),
                    )
                    .changed();
            });

            ui.collapsing("Displacement parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.displacement_scale, 0.0..=10.0)
                            .text("displacement scale"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.displacement_frequency, 0.0..=10.0)
                            .text("displacement frequency"),
                    )
                    .changed();
            });

            ui.collapsing("Global noise parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.global_noise_options.amplitude, 0.0..=2.0)
                            .text("global amplitude"),
                    )
                    .changed();
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.global_noise_options.frequency, 0.0..=2.0)
                            .text("global frequency"),
                    )
                    .changed();
                ui.horizontal(|ui| {
                    ui_changed |= ui
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
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.init_weight, 0.0..=1.)
                            .text("c.a. init noise weight"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.iterations, 0..=150)
                            .text("c.a. iterations"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.threshold, 0..=16)
                            .text("c.a. threshold"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.ca_options.search_radius, 0..=15)
                            .text("c.a. search radius"),
                    )
                    .changed();

                ui.horizontal(|ui| {
                    ui_changed |= ui
                        .add(egui::DragValue::new(&mut state.ca_options.seed))
                        .changed();
                    ui.label("seed");
                });

                ui_changed |= ui.checkbox(&mut state.invert_ca, "Invert").changed();

                ui.collapsing("Mask", |ui| {
                    ui_changed |= ui
                        .add(
                            egui::Slider::new(&mut state.ca_options.mask_options.mult, 0.0..=20.0)
                                .text("mult"),
                        )
                        .changed();
                    ui_changed |= ui
                        .add(
                            egui::Slider::new(&mut state.ca_options.mask_options.lift, 0.0..=20.0)
                                .text("lift"),
                        )
                        .changed();
                });
            });

            ui.collapsing("Post Processing", |ui| {
                ui_changed |= ui
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
                ui_changed |= ui
                    .radio_value(
                        &mut state.bitmap_dislpay,
                        variant.clone(),
                        format!("{:?}", variant),
                    )
                    .changed();
            }

            ui.add_space(smaller_space);

            ui_changed |= ui
                .checkbox(&mut state.show_texture, "Show texture")
                .changed();

            ui_changed |= ui
                .checkbox(&mut state.show_vectors, "Show Vectors")
                .changed();

            ui_changed |= ui.checkbox(&mut state.show_debug, "Show Debug").changed();

            ui.add_space(10.0);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Entity")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.scale, 5.0..=400.).text("scale"))
                .changed();

            ui.add_space(larger_space);

            ui_changed |= ui
                .checkbox(&mut state.gizmo_options.draw_gizmos, "Draw Gizmos")
                .changed();

            ui_changed |= ui.checkbox(&mut state.rooms, "Rooms").changed();



            ui.collapsing("Player", |ui| {
                ui_changed |= ui
                    .add(egui::Slider::new(&mut state.player_move_force, 0.0..=60.).text("move force"))
                    .changed();
                ui_changed |= ui
                    .add(egui::Slider::new(&mut state.player_jetpack_force, 0.0..=60.).text("jetpack force"))
                    .changed();
                ui_changed |= ui
                    .add(egui::Slider::new(&mut state.player_rotate_force, -10.0..=10.).text("rotate force"))
                    .changed();
            });

            ui.collapsing("Camera", |ui| {
                ui_changed |= ui
                    .add(egui::Checkbox::new(&mut state.game_camera, "game_cam"))
                    .changed();

                    ui_changed |= ui
                    .add(egui::Slider::new(&mut state.game_camera_zoom, -5.0..=50.).text("zoom"))
                    .changed();

                
            });



            if ui_changed {
                state.save().ok();
                event_writer.send(UiChangedEvent {
                    ui_state: state.clone(),
                });
            }






        })
        .response
        .rect
        .width();
}
