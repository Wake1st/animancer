use bevy::{math::vec2, prelude::*};

use crate::{
    generator::Generator,
    producer::{PostSpawnMarker, Producer, SPAWN_OFFSET},
    schedule::InGameSet,
    selectable::Selectable,
};

pub const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
pub const WORKER_PRODUCER_ASSET_PATH: &str = "worker producer.png";
pub const POST_SPAWN_MARKER_PATH: &str = "marker.png";
pub const SELECTION_SIZE: Vec2 = vec2(64., 64.);

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
    pub position: Vec3,
}

fn spawn_structure(
    mut placement_event: EventReader<PlaceStructure>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for place in placement_event.read() {
        let marker_texture: Handle<Image> = asset_server.load(POST_SPAWN_MARKER_PATH);
        let texture: Handle<Image> = asset_server.load(match place.structure_type {
            StructureType::SimpleShrine => SIMPLE_SHRINE_ASSET_PATH,
            StructureType::WorkerProducer => WORKER_PRODUCER_ASSET_PATH,
        });

        match place.structure_type {
            StructureType::SimpleShrine => {
                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_translation(place.position),
                        ..default()
                    },
                    Structure {},
                    Generator { ..default() },
                    Selectable {
                        size: SELECTION_SIZE,
                    },
                    Name::new("SimpleShrine"),
                ));
            }
            StructureType::WorkerProducer => {
                commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation(place.position),
                            ..default()
                        },
                        Structure {},
                        Producer {
                            post_spawn_location: place.position + SPAWN_OFFSET,
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
        }
    }
}
