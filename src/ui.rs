use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::{Event, EventWriter},
        system::{ResMut, Resource},
    },
};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use planet::types::FractalNoiseOptions;

pub struct PlanetUiPlugin;

impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        let ui_state = UiState::default();

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

// fn get_ui_dimensions_system(
//     mut contexts: EguiContexts,
//     mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
// ) {

//     let ctx = contexts.ctx_mut();

//     occupied_screen_space.left = egui::SidePanel::left("left_panel")
//     .resizable(true)
//     .show(ctx, |ui| {
//         ui.label("Left resizeable panel");
//         ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
//     })
//     .response
//     .rect
//     .width();
// }

fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut event_writer: EventWriter<UiChangedEvent>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let mut ui_changed = false;

    // state.noise.push(FractalNoiseOptions::default());
    // state.noise = vec![FractalNoiseOptions::default()];

    let a = 10.;
    let b = 5.;


    occupied_screen_space.left = egui::SidePanel::left("Hello")
        .default_width(500.)
        .show(contexts.ctx_mut(), |ui| {
            ui.style_mut().spacing.slider_width = 300.0;

            ui.add_space(a);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Noise")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(b);

            let mut noise_parameters_open = true;
            ui.collapsing("Noise 1 Parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[0].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[0].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[0].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[0].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[0].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();
            });
            ui.collapsing("Noise 2 Parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[1].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[1].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[1].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[1].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[1].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();
            });

            ui.collapsing("Noise 3 Parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[2].frequency, 0.0..=3.0)
                            .text("noise frequency"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[2].amplitude, 0.0..=1.0)
                            .text("noise amplitute"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[2].persistence, 0.0..=1.0)
                            .text("noise persistence"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[2].lacunarity, 1.0..=4.0)
                            .text("noise lacunarity"),
                    )
                    .changed();

                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.noise[2].octaves, 0..=10)
                            .text("noise octaves"),
                    )
                    .changed();
            });

            ui.collapsing("Mask parameters", |ui| {
                ui_changed |= ui
                    .add(
                        egui::Slider::new(&mut state.mask_frequency, 0.0..=3.0)
                            .text("mask frequency"),
                    )
                    .changed();
                ui_changed |= ui
                    .add(egui::Slider::new(&mut state.mask_z, 0.0..=20.0).text("mask z"))
                    .changed();
            });

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

            ui_changed |= ui
                .add(
                    egui::Slider::new(&mut state.global_amplitude, 0.0..=2.0)
                        .text("global amplitude"),
                )
                .changed();

            ui.add_space(a);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Initial")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(b);
            
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
                .add(egui::Slider::new(&mut state.resolution, 10..=800).text("resolution"))
                .changed();

            ui.add_space(a);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Cellular Automata")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(b);

            ui_changed |= ui
                .add(
                    egui::Slider::new(&mut state.ca_init_weight, 0.0..=1.)
                        .text("c.a. init noise weight"),
                )
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.ca_iterations, 0..=150).text("c.a. iterations"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.ca_thresh, 0..=16).text("c.a. threshold"))
                .changed();

                ui_changed |= ui
                .add(egui::Slider::new(&mut state.ca_misc, -16..=16).text("c.a. misc"))
                .changed();

            ui_changed |= ui
                .add(
                    egui::Slider::new(&mut state.ca_searh_radius, 0..=15)
                        .text("c.a. search radius"),
                )
                .changed();

                ui_changed |= ui
                .checkbox(&mut state.invert_ca, "Invert")
                .changed();

            ui.add_space(a);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Post Processing")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(b);

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.blur, 0.0..=8.).text("post blur"))
                .changed();

            ui.add_space(a);
            let heading_style = egui::TextStyle::Heading;
            let c = 180;
            ui.label(
                egui::RichText::new("Display")
                    .text_style(heading_style)
                    .color(egui::Color32::from_rgb(c, c, c)),
            );
            ui.add_space(b);

            ui_changed |= ui
                .radio_value(
                    &mut state.selected_option,
                    SelectedOption::Planet_raw,
                    "planet raw",
                )
                .changed();

            ui_changed |= ui
                .radio_value(
                    &mut state.selected_option,
                    SelectedOption::Planet_processed,
                    "planet processed",
                )
                .changed();

            ui_changed |= ui
                .radio_value(
                    &mut state.selected_option,
                    SelectedOption::Altitude,
                    "altitude",
                )
                .changed();

            ui_changed |= ui
                .radio_value(&mut state.selected_option, SelectedOption::Depth, "depth")
                .changed();

            ui_changed |= ui
                .radio_value(&mut state.selected_option, SelectedOption::Debug, "debug")
                .changed();

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
                .add(egui::Slider::new(&mut state.scale, 5.0..=100.).text("scale"))
                .changed();

            if ui_changed {
                event_writer.send(UiChangedEvent {
                    ui_state: state.clone(),
                });
            }
        })
        .response
        .rect
        .width();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectedOption {
    Planet_raw,
    Planet_processed,
    Altitude,
    Depth,
    Debug,
}

#[derive(Resource, Debug, Clone)]
pub struct UiState {
    pub changed: bool,
    pub noise: Vec<FractalNoiseOptions>,
    pub mask_frequency: f64,
    pub mask_z: f64,
    pub global_amplitude: f32,
    pub radius: f32,
    pub resolution: u32,
    pub ca_thresh: u32,
    pub ca_iterations: u32,
    pub ca_init_weight: f32,
    pub ca_searh_radius: u32,
    pub ca_misc: i32,
    pub blur: f32,
    pub selected_option: SelectedOption,
    pub scale: f32,
    pub show_texture: bool,
    pub show_vectors: bool,
    pub show_debug: bool,
    pub crust_thickness: f32,
    pub displacement_scale: f64,
    pub displacement_frequency: f64,
    pub invert_ca: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            changed: false,
            noise: vec![
                FractalNoiseOptions::default(),
                FractalNoiseOptions::default(),
                FractalNoiseOptions::default(),
            ],

            global_amplitude: 1.0,
            mask_frequency: 0.5,
            mask_z: 0.0,
            radius: 1.,
            resolution: 200,
            ca_thresh: 4,
            ca_iterations: 1,
            ca_init_weight: 0.62,
            ca_misc: 0,
            blur: 1.,
            scale: 100.,
            show_texture: true,
            show_vectors: true,
            show_debug: false,
            crust_thickness: 0.0,
            ca_searh_radius: 3,
            selected_option: SelectedOption::Planet_raw,
            displacement_scale: 0.0,
            displacement_frequency: 0.0,
            invert_ca: false,
        }
    }
}
