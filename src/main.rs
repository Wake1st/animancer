mod camera;
mod helpers;
mod inputs;
mod movement;
mod selectable;
mod structure;
mod unit;

use bevy::{log::LogPlugin, prelude::*};

use camera::CameraPlugin;
use helpers::HelperPlugin;
use inputs::InputPlugin;
use movement::MovementPlugin;
use selectable::SelectablePlugin;
use structure::StructurePlugin;
use unit::UnitPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Animancer".into(),
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
            InputPlugin,
            CameraPlugin,
            UnitPlugin,
            SelectablePlugin,
            MovementPlugin,
            HelperPlugin,
            StructurePlugin,
        ))
        .run();
}
