use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{
    faith::Faith,
    generator::{Generator, GeneratorType},
    selectable::Selectable,
};

const ASSET_PATH: &str = "harvester.png";
const SELECTION_SIZE: Vec2 = vec2(64., 64.);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_structure)
            .add_event::<PlaceStructure>();
    }
}

#[derive(Component)]
pub struct Structure {
    structure_type: StructureType,
    is_generating: bool,
    value: f32,
}

impl Default for Structure {
    fn default() -> Self {
        Self {
            structure_type: Default::default(),
            is_generating: Default::default(),
            value: Default::default(),
        }
    }
}

pub enum StructureType {
    SimpleShrine,
    WorkerProducer,
}

impl StructureType {
    fn get_generator_type(&self) -> GeneratorType {
        match self {
            Self::SimpleShrine => GeneratorType::Faith,
            Self::WorkerProducer => GeneratorType::Worker,
        }
    }
}

impl Default for StructureType {
    fn default() -> Self {
        StructureType::WorkerProducer
    }
}

impl Clone for StructureType {
    fn clone(&self) -> Self {
        match self {
            Self::SimpleShrine => Self::SimpleShrine,
            Self::WorkerProducer => Self::WorkerProducer,
        }
    }
}

#[derive(Event)]
pub struct PlaceStructure {
    structure_type: StructureType,
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
                structure_type: place.structure_type.clone(),
                ..Default::default()
            },
            Generator {
                gen_type: place.structure_type.get_generator_type(),
                is_running: true,
                value: 0.0,
                rate: 1.0,
                completion: 10.0,
            },
            Selectable {
                size: SELECTION_SIZE,
            },
            Name::new("Structure"),
        ));
    }
}
