use bevy::{math::vec2, prelude::*};

use crate::{
    ai::Idle,
    combat::Health,
    conversion::Faith,
    movement::{Moveable, Moving},
    nav_agent::{AssignNavigatorPath, Navigator},
    priest::Priest,
    producer::{Produce, ProductionType},
    schedule::InGameSet,
    selectable::Selectable,
    teams::{Team, TeamType},
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
        app.add_systems(Update, spawn_unit.in_set(InGameSet::SpawnEntities))
            .add_event::<UnitAction>();
    }
}

#[derive(Component, Default)]
pub struct Unit {}

#[derive(Event)]
pub struct UnitAction {
    pub position: Vec2,
}

pub fn spawn_hero(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    team: TeamType,
) {
    let texture: Handle<Image> = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        Unit {},
        Worker { effort: 4.5 },
        Health(120.0),
        Faith {
            base: 160.0,
            current: 160.0,
        },
        Moving(false),
        Moveable {
            location: Vec3::ZERO,
        },
        Selectable {
            size: vec2(32., 32.),
        },
        Navigator { speed: 120.0 },
        Team(team),
        Idle(true),
        Name::new("Hero"),
    ));
}

fn spawn_unit(
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
                Moving(false),
                Moveable {
                    location: event.location,
                },
                Selectable {
                    size: vec2(32., 32.),
                },
                Navigator { speed },
                Idle(true),
                Name::new(name),
            ))
            .insert(match event.production_type {
                ProductionType::Worker => {
                    (
                        Worker { effort: 1.5 },
                        Health(24.0),
                        Faith {
                            base: 44.0,
                            current: 44.0,
                        },
                    );
                }
                ProductionType::Priest => {
                    (
                        Priest { persuation: 3.0 },
                        Health(16.0),
                        Faith {
                            base: 76.0,
                            current: 76.0,
                        },
                    );
                }
                ProductionType::Warrior => {
                    (
                        Warrior { strength: 2.5 },
                        Health(42.0),
                        Faith {
                            base: 32.0,
                            current: 32.0,
                        },
                    );
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
