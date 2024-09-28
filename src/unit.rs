use bevy::{math::vec2, prelude::*};

use crate::{
    movement::Moveable,
    nav_agent::{AssignNavigatorPath, Navigator},
    producer::ProduceWorker,
    schedule::InGameSet,
    selectable::Selectable,
    worker::Worker,
};

const WORKER_SPEED: f32 = 100.0;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_worker.in_set(InGameSet::SpawnEntities))
            .add_event::<UnitAction>();
    }
}

#[derive(Component, Default)]
pub struct Unit {}

#[derive(Event)]
pub struct UnitAction {
    pub position: Vec2,
}

fn spawn_worker(
    mut production_event: EventReader<ProduceWorker>,
    mut nav_path_assigner: EventWriter<AssignNavigatorPath>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("footman.png");

    for event in production_event.read() {
        let id = commands
            .spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                Unit {},
                Worker { effort: 1.5 },
                Moveable {
                    location: event.location,
                },
                Selectable {
                    size: vec2(32., 32.),
                },
                Navigator {
                    speed: WORKER_SPEED,
                },
                Name::new("Worker"),
            ))
            .id();

        nav_path_assigner.send(AssignNavigatorPath {
            entity: id,
            location: event.location,
        });
    }
}
