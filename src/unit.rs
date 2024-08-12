use bevy::prelude::*;

use crate::{movement::Moveable, selectable::Selectable};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hero);
    }
}

#[derive(Component, Default)]
pub struct Unit {
    pub size: f32,
}

fn setup_hero(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Unit { size: 20.0 },
        Moveable {
            speed: 100.0,
            location: Vec3::ZERO,
        },
        Selectable { selected: false },
        Name::new("Hero"),
    ));
}
