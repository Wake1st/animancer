use bevy::{math::vec3, prelude::*};

use crate::{
    schedule::InGameSet,
    selectable::SelectedUnits,
    structure::{
        PlaceStructure, StructureType, SIMPLE_SHRINE_ASSET_PATH, WORKER_PRODUCER_ASSET_PATH,
    },
    unit::Unit,
};

const CONSTRUCTION_BOOST: f32 = 20.5;
const CONSTRUCTION_RANGE: f32 = 80.;
pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                place_structure,
                check_unit_working_range,
                increment_effort,
                place_building,
            )
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
    assigned_units: Vec<Entity>,
    working_units: Vec<Entity>,
}

#[derive(Event)]
pub struct PlaceConstructionSite {
    pub position: Vec2,
    pub structure_type: StructureType,
    pub effort: f32,
}

fn place_structure(
    mut placement_reader: EventReader<PlaceConstructionSite>,
    selected_units: Res<SelectedUnits>,
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
                assigned_units: selected_units.entities.clone(),
                working_units: Vec::new(),
            },
            Name::new("ConstructionSite"),
        ));
    }
}

fn check_unit_working_range(
    mut sites: Query<(&Transform, &mut ConstructionSite)>,
    units: Query<&Transform, With<Unit>>,
) {
    for (site_transform, mut site) in sites.iter_mut() {
        let mut swapping: Vec<(usize, Entity)> = vec![];

        //	store all the assigned units close enough to swap
        for (index, &entity) in site.assigned_units.iter().enumerate() {
            if let Ok(unit_transform) = units.get(entity) {
                if site_transform
                    .translation
                    .distance(unit_transform.translation)
                    < CONSTRUCTION_RANGE
                {
                    swapping.push((index, entity));
                }
            }
        }

        //	swap the assigned units to working units
        for (index, entity) in swapping.iter() {
            site.working_units.push(*entity);
            site.assigned_units.remove(*index);
        }
    }
}

fn increment_effort(mut sites: Query<&mut ConstructionSite>, time: Res<Time>) {
    for mut site in &mut sites.iter_mut() {
        site.effort -= time.delta_seconds() * CONSTRUCTION_BOOST * site.working_units.len() as f32;
    }
}

fn place_building(
    sites: Query<(Entity, &GlobalTransform, &ConstructionSite)>,
    mut build_event: EventWriter<PlaceStructure>,
    mut commands: Commands,
) {
    for (entity, transform, site) in sites.iter() {
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
