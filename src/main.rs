mod camera;
mod combat;
mod construction;
mod conversion;
mod currency;
mod generator;
mod helpers;
mod inputs;
mod movement;
mod nav_agent;
mod priest;
mod producer;
mod schedule;
mod selectable;
mod state;
mod structure;
mod teams;
mod test_scene;
mod ui;
mod unit;
mod visual_feedback;
mod warrior;
mod worker;

use bevy::{log::LogPlugin, prelude::*, render::primitives::Aabb, window::WindowMode};

use camera::CameraPlugin;
use combat::CombatPlugin;
use construction::ConstructionPlugin;
use conversion::ConversionPlugin;
use currency::CurrencyPlugin;
use generator::GeneratorPlugin;
use helpers::HelperPlugin;
use inputs::InputPlugin;
use movement::MovementPlugin;
use nav_agent::{NavAgentPlugin, Obstacle};
use producer::ProducerPlugin;
use schedule::SchedulePlugin;
use selectable::SelectablePlugin;
use state::StatePlugin;
use structure::StructurePlugin;
use test_scene::TestScenePlugin;
use ui::UIPlugin;
use unit::UnitPlugin;
use visual_feedback::VisualFeedbackPlugin;
use vleue_navigator::{prelude::NavmeshUpdaterPlugin, VleueNavigatorPlugin};
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
            VleueNavigatorPlugin,
            // Auto update the navmesh.
            // Obstacles will be entities with the `Obstacle` marker component,
            // and use the `Aabb` component as the obstacle data source.
            NavmeshUpdaterPlugin::<Aabb, Obstacle>::default(),
            NavAgentPlugin,
        ))
        .add_plugins((
            StatePlugin,
            SchedulePlugin,
            UIPlugin,
            InputPlugin,
            CameraPlugin,
            VisualFeedbackPlugin,
        ))
        .add_plugins((
            CurrencyPlugin,
            UnitPlugin,
            SelectablePlugin,
            MovementPlugin,
            ConstructionPlugin,
            StructurePlugin,
            GeneratorPlugin,
            ProducerPlugin,
            WorkerPlugin,
            CombatPlugin,
            ConversionPlugin,
        ))
        .add_plugins((TestScenePlugin, HelperPlugin))
        .run();
}
