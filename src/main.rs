mod camera;
mod construction;
mod currency;
mod generator;
mod helpers;
mod inputs;
mod movement;
mod producer;
mod schedule;
mod selectable;
mod state;
mod structure;
mod test_scene;
mod ui;
mod unit;
mod worker;

use bevy::{log::LogPlugin, prelude::*, window::WindowMode};

use camera::CameraPlugin;
use construction::ConstructionPlugin;
use currency::CurrencyPlugin;
use generator::GeneratorPlugin;
use helpers::HelperPlugin;
use inputs::InputPlugin;
use movement::MovementPlugin;
use producer::ProducerPlugin;
use schedule::SchedulePlugin;
use selectable::SelectablePlugin;
use state::StatePlugin;
use structure::StructurePlugin;
use test_scene::TestScenePlugin;
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
            StructurePlugin,
            ConstructionPlugin,
            GeneratorPlugin,
            ProducerPlugin,
            CurrencyPlugin,
            WorkerPlugin,
        ))
        .add_plugins((TestScenePlugin, HelperPlugin))
        .run();
}
