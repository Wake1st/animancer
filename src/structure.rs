use bevy::{math::vec2, prelude::*};
use vleue_navigator::prelude::PrimitiveObstacle;

use crate::{
    ai::Idle,
    combat::Health,
    generator::Generator,
    producer::{
        PostSpawnMarker, Producer, Production, ProductionType, PRIEST_COST, SPAWN_OFFSET,
        WARRIOR_COST, WORKER_COST,
    },
    schedule::InGameSet,
    selectable::Selectable,
    teams::{Team, TeamType},
};

pub const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
pub const PRODUCER_ASSET_PATH: &str = "worker producer.png";
pub const POST_SPAWN_MARKER_PATH: &str = "marker.png";
pub const SELECTION_SIZE: Vec2 = vec2(64., 64.);
pub const SIMPLE_SHRINE_HEALTH: f32 = 660.0;
pub const PRODUCER_HEALTH: f32 = 1400.0;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_structure.in_set(InGameSet::EntityUpdates))
            .add_event::<PlaceStructure>();
    }
}

#[derive(Component)]
pub struct Structure;

#[derive(PartialEq, Debug)]
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
    pub team: TeamType,
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
                    PrimitiveObstacle::Rectangle(Rectangle::from_corners(
                        Vec2::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y) / 2.0,
                        Vec2::new(SELECTION_SIZE.x, SELECTION_SIZE.y) / 2.0,
                    )),
                    Structure {},
                    Health(SIMPLE_SHRINE_HEALTH),
                    Generator { ..default() },
                    Selectable {
                        size: SELECTION_SIZE,
                    },
                    Idle(true),
                    Team(place.team.clone()),
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
                        PrimitiveObstacle::Rectangle(Rectangle::from_corners(
                            Vec2::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y) / 2.0,
                            Vec2::new(SELECTION_SIZE.x, SELECTION_SIZE.y) / 2.0,
                        )),
                        Structure {},
                        Health(PRODUCER_HEALTH),
                        Producer {
                            post_spawn_location: place.position + SPAWN_OFFSET,
                            ..default()
                        },
                        Selectable {
                            size: SELECTION_SIZE,
                        },
                        Idle(true),
                        Team(place.team.clone()),
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

                        builder.spawn(Production {
                            production_type: ProductionType::Worker,
                            cost: WORKER_COST,
                            queue: 0,
                        });
                        builder.spawn(Production {
                            production_type: ProductionType::Priest,
                            cost: PRIEST_COST,
                            queue: 0,
                        });
                        builder.spawn(Production {
                            production_type: ProductionType::Warrior,
                            cost: WARRIOR_COST,
                            queue: 0,
                        });
                    });
            }
        }
    }
}
