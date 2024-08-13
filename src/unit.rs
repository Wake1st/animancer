use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{movement::Moveable, selectable::Selectable};

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
            selected: false,
            size: vec2(32., 32.),
        },
        Name::new("Hero"),
    ));
}

fn spawn_worker(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("footman.png");

    let iter = 0..40;
    iter.for_each(|i| {
        let location = vec3((i % 10) as f32 * 60., (i / 10) as f32 * 60., 0.0);

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(location),
                ..default()
            },
            Unit {},
            Moveable {
                speed: 100.0,
                location,
            },
            Selectable {
                selected: false,
                size: vec2(32., 32.),
            },
            Name::new("Worker"),
        ));
    });
}
