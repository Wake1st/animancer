use bevy::{math::vec2, prelude::*, render::primitives::Aabb};

use crate::{
    generator::Generator,
    nav_agent::Obstacle,
    producer::{
        PostSpawnMarker, Producer, Production, ProductionType, PRIEST_COST, SPAWN_OFFSET,
        WORKER_COST,
    },
    schedule::InGameSet,
    selectable::Selectable,
};

pub const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
pub const PRODUCER_ASSET_PATH: &str = "worker producer.png";
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

#[derive(PartialEq)]
pub enum StructureType {
    SimpleShrine,
    Producer,
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
            Self::Producer => Self::Producer,
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
            StructureType::Producer => PRODUCER_ASSET_PATH,
        });

        match place.structure_type {
            StructureType::SimpleShrine => {
                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_translation(place.position),
                        ..default()
                    },
                    Obstacle,
                    Aabb::from_min_max(
                        Vec3::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y, 0.0),
                        Vec3::new(SELECTION_SIZE.x, SELECTION_SIZE.y, 0.0),
                    ),
                    Structure {},
                    Generator { ..default() },
                    Selectable {
                        size: SELECTION_SIZE,
                    },
                    Name::new("SimpleShrine"),
                ));
            }
            StructureType::Producer => {
                commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation(place.position),
                            ..default()
                        },
                        Obstacle,
                        Aabb::from_min_max(
                            Vec3::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y, 0.0),
                            Vec3::new(SELECTION_SIZE.x, SELECTION_SIZE.y, 0.0),
                        ),
                        Structure {},
                        Producer {
                            productions: vec![
                                Production {
                                    production_type: ProductionType::Worker,
                                    cost: WORKER_COST,
                                    queue: 0,
                                },
                                Production {
                                    production_type: ProductionType::Priest,
                                    cost: PRIEST_COST,
                                    queue: 0,
                                },
                            ],
                            post_spawn_location: place.position + SPAWN_OFFSET,
                            ..default()
                        },
                        Selectable {
                            size: SELECTION_SIZE,
                        },
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
                    });
            }
        }
    }
}
