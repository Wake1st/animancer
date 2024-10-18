use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::primitives::Aabb,
};

use crate::{
    combat::Health,
    conversion::Faith,
    generator::Generator,
    movement::Moveable,
    nav_agent::{Navigator, Obstacle},
    priest::Priest,
    producer::{
        PostSpawnMarker, Producer, Production, ProductionType, PRIEST_COST, SPAWN_OFFSET,
        WARRIOR_COST, WORKER_COST,
    },
    selectable::Selectable,
    structure::{
        Structure, POST_SPAWN_MARKER_PATH, PRODUCER_ASSET_PATH, PRODUCER_HEALTH, SELECTION_SIZE,
        SIMPLE_SHRINE_ASSET_PATH, SIMPLE_SHRINE_HEALTH,
    },
    teams::{Team, TeamType},
    unit::Unit,
    unit::{GOOD_PRIEST_ASSET_PATH, GOOD_WARRIOR_ASSET_PATH, GOOD_WORKER_ASSET_PATH},
    warrior::Warrior,
    worker::Worker,
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_hero::<true>,
                spawn_workers::<true>,
                spawn_priests::<true>,
                spawn_warriors::<true>,
                spawn_structures::<true>,
                spawn_hero::<false>,
                spawn_workers::<false>,
                spawn_priests::<false>,
                spawn_warriors::<false>,
                spawn_structures::<false>,
            ),
        );
    }
}

fn spawn_hero<const IS_HUMAN: bool>(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("character.png");
    let team_side = if IS_HUMAN { -1.0 } else { 1.0 };
    let spawn_position_base = vec3(0., 200. * team_side, 0.);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(spawn_position_base),
            ..default()
        },
        Unit {},
        Worker { effort: 4.5 },
        Health(120.0),
        Faith {
            base: 160.0,
            current: 160.0,
        },
        Moveable {
            location: Vec3::ZERO,
        },
        Selectable {
            size: vec2(32., 32.),
        },
        Navigator { speed: 120.0 },
        Team(if IS_HUMAN {
            TeamType::Human
        } else {
            TeamType::CPU
        }),
        Name::new("Hero"),
    ));
}

fn spawn_workers<const IS_HUMAN: bool>(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load(GOOD_WORKER_ASSET_PATH);
    let team_side = if IS_HUMAN { -1.0 } else { 1.0 };
    let spawn_position_base = vec3(-100., 200. * team_side, 0.);

    for n in 0..10 {
        let position = spawn_position_base + vec3(30. * (n % 2) as f32, 30. * (n / 2) as f32, 0.);
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Unit {},
            Worker { effort: 1.5 },
            Health(24.0),
            Faith {
                base: 44.,
                current: 44.,
            },
            Moveable { location: position },
            Selectable {
                size: vec2(32., 32.),
            },
            Navigator { speed: 100.0 },
            Team(if IS_HUMAN {
                TeamType::Human
            } else {
                TeamType::CPU
            }),
            Name::new("Worker"),
        ));
    }
}

fn spawn_priests<const IS_HUMAN: bool>(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load(GOOD_PRIEST_ASSET_PATH);
    let team_side = if IS_HUMAN { -1.0 } else { 1.0 };
    let spawn_position_base = vec3(-200., 200. * team_side, 0.);

    for n in 0..10 {
        let position = spawn_position_base + vec3(30. * (n % 2) as f32, 30. * (n / 2) as f32, 0.);
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Unit {},
            Priest { persuation: 3.0 },
            Health(16.0),
            Faith {
                base: 76.,
                current: 76.,
            },
            Moveable { location: position },
            Selectable {
                size: vec2(32., 32.),
            },
            Navigator { speed: 85.0 },
            Team(if IS_HUMAN {
                TeamType::Human
            } else {
                TeamType::CPU
            }),
            Name::new("Priest"),
        ));
    }
}

fn spawn_warriors<const IS_HUMAN: bool>(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load(GOOD_WARRIOR_ASSET_PATH);
    let team_side = if IS_HUMAN { -1.0 } else { 1.0 };
    let spawn_position_base = vec3(-300., 200. * team_side, 0.);

    for n in 0..10 {
        let position = spawn_position_base + vec3(30. * (n % 2) as f32, 30. * (n / 2) as f32, 0.);
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Unit {},
            Warrior { strength: 2.5 },
            Health(42.0),
            Faith {
                base: 32.0,
                current: 32.0,
            },
            Moveable { location: position },
            Selectable {
                size: vec2(32., 32.),
            },
            Navigator { speed: 85.0 },
            Team(if IS_HUMAN {
                TeamType::Human
            } else {
                TeamType::CPU
            }),
            Name::new("Priest"),
        ));
    }
}

fn spawn_structures<const IS_HUMAN: bool>(asset_server: Res<AssetServer>, mut commands: Commands) {
    //  SIMPLE SHRINE
    let marker_texture: Handle<Image> = asset_server.load(POST_SPAWN_MARKER_PATH);
    let simple_shrine_texture: Handle<Image> = asset_server.load(SIMPLE_SHRINE_ASSET_PATH);
    let team_side = if IS_HUMAN { -1.0 } else { 1.0 };
    let mut spawn_position_base = vec3(100., 200. * team_side, 0.);

    commands.spawn((
        SpriteBundle {
            texture: simple_shrine_texture,
            transform: Transform::from_translation(spawn_position_base),
            ..default()
        },
        Obstacle,
        Aabb::from_min_max(
            Vec3::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y, 0.0),
            Vec3::new(SELECTION_SIZE.x, SELECTION_SIZE.y, 0.0),
        ),
        Structure {},
        Health(SIMPLE_SHRINE_HEALTH),
        Generator { ..default() },
        Selectable {
            size: SELECTION_SIZE,
        },
        Team(if IS_HUMAN {
            TeamType::Human
        } else {
            TeamType::CPU
        }),
        Name::new("SimpleShrine"),
    ));

    //  PRODUCER
    let producer_texture: Handle<Image> = asset_server.load(PRODUCER_ASSET_PATH);
    spawn_position_base += vec3(100., 0.0, 0.);
    commands
        .spawn((
            SpriteBundle {
                texture: producer_texture,
                transform: Transform::from_translation(spawn_position_base),
                ..default()
            },
            Obstacle,
            Aabb::from_min_max(
                Vec3::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y, 0.0),
                Vec3::new(SELECTION_SIZE.x, SELECTION_SIZE.y, 0.0),
            ),
            Structure {},
            Health(PRODUCER_HEALTH),
            Producer {
                post_spawn_location: spawn_position_base + SPAWN_OFFSET,
                ..default()
            },
            Selectable {
                size: SELECTION_SIZE,
            },
            Team(if IS_HUMAN {
                TeamType::Human
            } else {
                TeamType::CPU
            }),
            Name::new("Producer"),
        ))
        .with_children(|builder| {
            builder.spawn((
                SpriteBundle {
                    texture: marker_texture,
                    transform: Transform::from_translation(SPAWN_OFFSET),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                PostSpawnMarker { not_set: true },
                Name::new("PostSpawnMarker"),
            ));

            builder.spawn(Production {
                production_type: ProductionType::Worker,
                cost: WORKER_COST,
                queue: 0,
            });
            builder.spawn(Production {
                production_type: ProductionType::Priest,
                cost: PRIEST_COST,
                queue: 0,
            });
            builder.spawn(Production {
                production_type: ProductionType::Warrior,
                cost: WARRIOR_COST,
                queue: 0,
            });
        });
}
