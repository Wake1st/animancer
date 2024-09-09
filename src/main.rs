mod camera;
mod faith;
mod generator;
mod helpers;
mod inputs;
mod movement;
mod schedule;
mod selectable;
mod state;
mod structure;
mod ui;
mod unit;
mod worker;

use bevy::{log::LogPlugin, prelude::*, window::WindowMode};

use camera::CameraPlugin;
use faith::FaithPlugin;
use generator::GeneratorPlugin;
use helpers::HelperPlugin;
use inputs::InputPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use selectable::SelectablePlugin;
use state::StatePlugin;
use structure::StructurePlugin;
use ui::UIPlugin;
use unit::UnitPlugin;
use worker::WorkerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Animancer".into(),
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                })
                .build(),
        )
        .add_plugins((
            StatePlugin,
            SchedulePlugin,
            UIPlugin,
            InputPlugin,
            CameraPlugin,
            UnitPlugin,
            SelectablePlugin,
            MovementPlugin,
            HelperPlugin,
            StructurePlugin,
            GeneratorPlugin,
            FaithPlugin,
            WorkerPlugin,
        ))
        .run();
}
