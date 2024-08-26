use bevy::{math::vec2, prelude::*};

use crate::{generator::GenerateWorker, movement::Moveable, selectable::Selectable};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_hero, spawn_worker));
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
    mut generation_event: EventReader<GenerateWorker>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("footman.png");

    for event in generation_event.read() {
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(event.position),
                ..default()
            },
            Unit {},
            Moveable {
                speed: 100.0,
                location: event.position,
            },
            Selectable {
                size: vec2(32., 32.),
            },
            Name::new("Worker"),
        ));
    }
}
