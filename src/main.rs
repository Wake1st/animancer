mod camera;
mod inputs;
mod movement;
mod selectable;
mod unit;

use bevy::{log::LogPlugin, prelude::*};

use camera::CameraPlugin;
use inputs::InputPlugin;
use movement::MovementPlugin;
use selectable::SelectablePlugin;
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
        ))
        .run();
}
