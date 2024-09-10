use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{
    generator::{Generator, GeneratorType},
    producer::Producer,
    schedule::InGameSet,
    selectable::Selectable,
};

const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
const WORKER_PRODUCER_ASSET_PATH: &str = "worker producer.png";
const SELECTION_SIZE: Vec2 = vec2(64., 64.);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_structure.in_set(InGameSet::EntityUpdates))
            .add_event::<PlaceStructure>();
    }
}

#[derive(Component)]
pub struct Structure;

pub enum StructureType {
    SimpleShrine,
    WorkerProducer,
}

impl Default for StructureType {
    fn default() -> Self {
        StructureType::SimpleShrine
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
    pub structure_type: StructureType,
    pub position: Vec2,
}

fn spawn_structure(
    mut placement_event: EventReader<PlaceStructure>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for place in placement_event.read() {
        let texture: Handle<Image> = asset_server.load(match place.structure_type {
            StructureType::SimpleShrine => SIMPLE_SHRINE_ASSET_PATH,
            StructureType::WorkerProducer => WORKER_PRODUCER_ASSET_PATH,
        });

        match place.structure_type {
            StructureType::SimpleShrine => {
                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_translation(vec3(
                            place.position.x,
                            place.position.y,
                            0.0,
                        )),
                        ..default()
                    },
                    Structure {},
                    Generator {
                        gen_type: GeneratorType::Faith,
                        is_running: true,
                        rate: 1.0,
                    },
                    Selectable {
                        size: SELECTION_SIZE,
                    },
                    Name::new("SimpleShrine"),
                ));
            }
            StructureType::WorkerProducer => {
                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_translation(vec3(
                            place.position.x,
                            place.position.y,
                            0.0,
                        )),
                        ..default()
                    },
                    Structure {},
                    Producer {
                        queue: 0,
                        cost: 10.,
                        value: 0.,
                        rate: 2.5,
                    },
                    Selectable {
                        size: SELECTION_SIZE,
                    },
                    Name::new("WorkerProducer"),
                ));
            }
        }
    }
}
