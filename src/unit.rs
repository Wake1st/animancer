use bevy::{math::vec2, prelude::*};

use crate::{
    combat::Health,
    movement::Moveable,
    nav_agent::{AssignNavigatorPath, Navigator},
    priest::Priest,
    producer::{Produce, ProductionType},
    schedule::InGameSet,
    selectable::Selectable,
    ui::{PRIEST_ASSET_PATH, WARRIOR_ASSET_PATH, WORKER_ASSET_PATH},
    warrior::Warrior,
    worker::Worker,
};

const WORKER_SPEED: f32 = 100.0;
const PRIEST_SPEED: f32 = 85.0;
const WARRIOR_SPEED: f32 = 120.0;

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
    mut production_event: EventReader<Produce>,
    mut nav_path_assigner: EventWriter<AssignNavigatorPath>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in production_event.read() {
        let (texture_path, speed, name) = match event.production_type {
            ProductionType::Worker => (WORKER_ASSET_PATH, WORKER_SPEED, "Worker"),
            ProductionType::Priest => (PRIEST_ASSET_PATH, PRIEST_SPEED, "Priest"),
            ProductionType::Warrior => (WARRIOR_ASSET_PATH, WARRIOR_SPEED, "Warrior"),
            ProductionType::None => ("", 0.0, "None"),
        };
        let texture = asset_server.load(texture_path);

        let id = commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                Unit {},
                Moveable {
                    location: event.location,
                },
                Selectable {
                    size: vec2(32., 32.),
                },
                Navigator { speed },
                Name::new(name),
            ))
            .insert(match event.production_type {
                ProductionType::Worker => {
                    (Worker { effort: 1.5 }, Health(24.0));
                }
                ProductionType::Priest => {
                    (Priest { persuation: 3.0 }, Health(16.0));
                }
                ProductionType::Warrior => {
                    (Warrior { strength: 2.5 }, Health(42.0));
                }
                ProductionType::None => {
                    todo!();
                }
            })
            .id();

        nav_path_assigner.send(AssignNavigatorPath {
            entity: id,
            location: event.location + Vec3::new(0.0, -1.0, 0.0),
        });
    }
}
