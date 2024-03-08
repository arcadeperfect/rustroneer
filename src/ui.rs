use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::{ResMut, Resource},
    },
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{bevy_planet, types::UiState};

pub struct PlanetUiPlugin;

impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        let ui_state = UiState::default();

        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(bevy_planet::PlanetPlugin)
            .insert_resource(ui_state)
            .add_event::<UiChangedEvent>()
            .add_systems(Startup, init_planet_system)
            .add_systems(Update, ui_system)
            .add_event::<UiChangedEvent>();
    }
}

#[derive(Event, Debug)]
pub struct UiChangedEvent {
    pub ui_state: UiState,
}

fn init_planet_system(mut state: ResMut<UiState>, mut event_writer: EventWriter<UiChangedEvent>) {
    event_writer.send(UiChangedEvent {
        ui_state: state.clone(),
    });
}

fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut event_writer: EventWriter<UiChangedEvent>,
) {
    let mut ui_changed = false;

    egui::SidePanel::left("Hello")
        .default_width(500.)
        .show(contexts.ctx_mut(), |ui| {
            ui.style_mut().spacing.slider_width = 300.0;
            ui_changed |= ui
                .add(egui::Slider::new(&mut state.frequency, 0.0..=1.0).text("noise frequency"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.amplitude, 0.0..=1.0).text("noise amplitute"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.radius, 0.0..=1.0).text("circle radius"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.resolution, 10..=800).text("resolution"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.weight, 0.0..=1.).text("c.a. init noise weight"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.iterations, 0..=150).text("c.a. iterations"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.thresh, 0..=8).text("c.a. threshold"))
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.blur, 0.0..=8.).text("post blur"))
                .changed();

            ui_changed |= ui
                .radio_value(
                    &mut state.selected_option,
                    "planet".to_string(),
                    "planet buffer",
                )
                .changed();

            ui_changed |= ui
                .radio_value(
                    &mut state.selected_option,
                    "data".to_string(),
                    "data buffer",
                )
                .changed();

            ui_changed |= ui
                .radio_value(
                    &mut state.selected_option,
                    "debug".to_string(),
                    "debug buffer",
                )
                .changed();

            ui_changed |= ui
                .add(egui::Slider::new(&mut state.thresh, 1..=10).text("scale"))
                .changed();

            if ui_changed {
                event_writer.send(UiChangedEvent {
                    ui_state: state.clone(),
                });
            }
        });
}
