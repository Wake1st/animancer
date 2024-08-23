use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::selectable::Selectable;

const ASSET_PATH: &str = "harvester.png";
const SELECTION_SIZE: Vec2 = vec2(64., 64.);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_structure)
            .add_event::<PlaceStructure>();
    }
}

#[derive(Component, Default)]
pub struct Structure {
    structure_type: StructureType,
}

pub enum StructureType {
    WorkerSpawner,
}

impl Default for StructureType {
    fn default() -> Self {
        StructureType::WorkerSpawner
    }
}

#[derive(Event)]
pub struct PlaceStructure {
    pos: Vec2,
}

fn spawn_structure(
    mut placement_event: EventReader<PlaceStructure>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for place in placement_event.read() {
        let texture: Handle<Image> = asset_server.load(ASSET_PATH);

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(vec3(place.pos.x, place.pos.y, 0.0)),
                ..default()
            },
            Structure {
                structure_type: Default::default(),
            },
            Selectable {
                size: SELECTION_SIZE,
            },
            Name::new("Structure"),
        ));
    }
}
