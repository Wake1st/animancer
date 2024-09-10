use bevy::{math::vec2, prelude::*};

use crate::{movement::Moveable, producer::ProduceWorker, selectable::Selectable, worker::Worker};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hero)
            .add_systems(Update, spawn_worker);
    }
}

#[derive(Component, Default)]
pub struct Unit {}

fn spawn_hero(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Unit {},
        Worker {},
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

fn spawn_worker(
    mut production_event: EventReader<ProduceWorker>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("footman.png");

    for event in production_event.read() {
        info!("worker spawning at : {:?}", event.position);
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(event.position),
                ..default()
            },
            Unit {},
            Worker {},
            Moveable {
                speed: 100.0,
                location: event.location,
            },
            Selectable {
                size: vec2(32., 32.),
            },
            Name::new("Worker"),
        ));
    }
}
