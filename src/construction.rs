use bevy::{math::vec3, prelude::*};

use crate::{
    schedule::InGameSet,
    structure::{
        PlaceStructure, StructureType, SIMPLE_SHRINE_ASSET_PATH, WORKER_PRODUCER_ASSET_PATH,
    },
};

const CONSTRUCTION_BOOST: f32 = 2.5;

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (place_structure, increment_effort, place_building)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<PlaceConstructionSite>();
    }
}

#[derive(Component)]
struct ConstructionSite {
    structure_type: StructureType,
    effort: f32,
}

#[derive(Event)]
pub struct PlaceConstructionSite {
    pub position: Vec2,
    pub structure_type: StructureType,
    pub effort: f32,
}

fn place_structure(
    mut placement_reader: EventReader<PlaceConstructionSite>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for placement in placement_reader.read() {
        let texture: Handle<Image> = asset_server.load(match placement.structure_type {
            StructureType::SimpleShrine => SIMPLE_SHRINE_ASSET_PATH,
            StructureType::WorkerProducer => WORKER_PRODUCER_ASSET_PATH,
        });

        let pos_3d = vec3(placement.position.x, placement.position.y, 0.0);

        commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    color: Color::linear_rgba(0.0, 0.1, 0.1, 0.6),
                    ..default()
                },
                transform: Transform::from_translation(pos_3d),
                ..default()
            },
            ConstructionSite {
                structure_type: placement.structure_type.clone(),
                effort: placement.effort,
            },
            Name::new("ConstructionSite"),
        ));
    }
}

fn increment_effort(mut construction_sites: Query<&mut ConstructionSite>, time: Res<Time>) {
    for mut site in &mut construction_sites.iter_mut() {
        site.effort -= time.delta_seconds() * CONSTRUCTION_BOOST;
    }
}

fn place_building(
    construction_sites: Query<(Entity, &GlobalTransform, &ConstructionSite)>,
    mut build_event: EventWriter<PlaceStructure>,
    mut commands: Commands,
) {
    for (entity, transform, site) in construction_sites.iter() {
        if site.effort < 0.0 {
            build_event.send(PlaceStructure {
                structure_type: site.structure_type.clone(),
                position: transform.translation(),
            });

            //	destroy the site after building
            commands.entity(entity).despawn_recursive();
        }
    }
}
