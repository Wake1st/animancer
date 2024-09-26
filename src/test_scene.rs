use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{
    generator::Generator,
    movement::Moveable,
    producer::{PostSpawnMarker, Producer, SPAWN_OFFSET},
    selectable::Selectable,
    structure::{
        Structure, POST_SPAWN_MARKER_PATH, SELECTION_SIZE, SIMPLE_SHRINE_ASSET_PATH,
        WORKER_PRODUCER_ASSET_PATH,
    },
    unit::Unit,
    worker::Worker,
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_hero, spawn_workers, spawn_structures));
    }
}

fn spawn_hero(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Unit {},
        Worker { effort: 4.5 },
        Moveable {
            speed: 100.0,
            location: Vec3::ZERO,
        },
        Selectable {
            size: vec2(32., 32.),
        },
        Name::new("Hero"),
    ));
}

fn spawn_workers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("footman.png");
    let spawn_position_base = vec3(200., -200., 0.);

    for n in 0..20 {
        let position = spawn_position_base + vec3(30. * (n % 5) as f32, 30. * (n / 5) as f32, 0.);
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Unit {},
            Worker { effort: 1.5 },
            Moveable {
                speed: 100.0,
                location: position,
            },
            Selectable {
                size: vec2(32., 32.),
            },
            Name::new("Worker"),
        ));
    }
}

fn spawn_structures(asset_server: Res<AssetServer>, mut commands: Commands) {
    //  SIMPLE SHRINE
    let marker_texture: Handle<Image> = asset_server.load(POST_SPAWN_MARKER_PATH);
    let simple_shrine_texture: Handle<Image> = asset_server.load(SIMPLE_SHRINE_ASSET_PATH);

    let mut spawn_position_base = vec3(200., 200., 0.);
    commands.spawn((
        SpriteBundle {
            texture: simple_shrine_texture,
            transform: Transform::from_translation(spawn_position_base),
            ..default()
        },
        Structure {},
        Generator { ..default() },
        Selectable {
            size: SELECTION_SIZE,
        },
        Name::new("SimpleShrine"),
    ));

    //  WORKER PRODUCER
    let worker_producer_texture: Handle<Image> = asset_server.load(WORKER_PRODUCER_ASSET_PATH);
    spawn_position_base += vec3(100., 0., 0.);
    commands
        .spawn((
            SpriteBundle {
                texture: worker_producer_texture,
                transform: Transform::from_translation(spawn_position_base),
                ..default()
            },
            Structure {},
            Producer {
                post_spawn_location: spawn_position_base + SPAWN_OFFSET,
                ..default()
            },
            Selectable {
                size: SELECTION_SIZE,
            },
            Name::new("WorkerProducer"),
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
        });
}
