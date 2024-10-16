use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::primitives::Aabb,
};
use vleue_navigator::prelude::PrimitiveObstacle;

use crate::{
    ai::Idle,
    currency::Energy,
    inputs::{BuildSelection, MousePosition},
    movement::{Formation, SetUnitPosition},
    nav_agent::Obstacle,
    schedule::InGameSet,
    selectable::{Selectable, SelectedUnits, SelectionStateChanged, SelectionType},
    structure::{
        PlaceStructure, StructureType, PRODUCER_ASSET_PATH, SELECTION_SIZE,
        SIMPLE_SHRINE_ASSET_PATH,
    },
    teams::TeamType,
    unit::Unit,
};

const CONSTRUCTION_BOOST: f32 = 20.5;
pub const CONSTRUCTION_RANGE: f32 = 90.;
const BUILD_APPROVED_COLOR: Color = Color::linear_rgba(0.1, 0.7, 0.0, 0.4);
const BUILD_DENIED_COLOR: Color = Color::linear_rgba(0.7, 0.1, 0.0, 0.4);

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_construction_silhouette,
                (attempt_construction_placement, place_construction_site).chain(),
                assign_new_workers,
                (set_assigned_units, set_working_units).chain(),
                increment_effort,
                place_structure,
                (display_construction_silhouette, display_site_validity).chain(),
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<AttemptSitePlacement>()
        .add_event::<PlaceConstructionSite>()
        .add_event::<AssignConstructionWorkers>();
    }
}

#[derive(Component)]
pub struct ConstructionSilhouette {}

#[derive(Component)]
pub struct Intersects(pub bool);

#[derive(Component)]
pub struct ConstructionSite {
    structure_type: StructureType,
    team: TeamType,
    effort: f32,
    assigned_units: Vec<Entity>,
    working_units: Vec<Entity>,
}

#[derive(Event)]
pub struct AttemptSitePlacement {
    pub position: Vec2,
    pub team: TeamType,
}

#[derive(Event)]
pub struct PlaceConstructionSite {
    pub structure_type: StructureType,
    pub team: TeamType,
    pub position: Vec2,
    pub effort: f32,
}

#[derive(Event)]
pub struct AssignConstructionWorkers {
    pub site: Entity,
    pub units: Vec<Entity>,
}

fn display_construction_silhouette(
    mut selection_state_updated: EventReader<SelectionStateChanged>,
    mut silhouettes: Query<Entity, With<ConstructionSilhouette>>,
    build_selection: Res<BuildSelection>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in selection_state_updated.read() {
        //  will always need to remove upon change, even if the new state is still construction
        for entity in silhouettes.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        if event.new_type == SelectionType::Construction {
            let texture: Handle<Image> = asset_server.load(match build_selection.structure_type {
                StructureType::SimpleShrine => SIMPLE_SHRINE_ASSET_PATH,
                StructureType::Producer => PRODUCER_ASSET_PATH,
            });

            commands.spawn((
                SpriteBundle {
                    texture,
                    sprite: Sprite {
                        color: Color::linear_rgba(0.1, 0.1, 0.1, 0.2),
                        ..default()
                    },
                    ..default()
                },
                ConstructionSilhouette {},
                Intersects(false),
            ));
        }
    }
}

fn move_construction_silhouette(
    mut silhouettes: Query<&mut Transform, With<ConstructionSilhouette>>,
    mouse_position: Res<MousePosition>,
) {
    for mut transform in silhouettes.iter_mut() {
        transform.translation = mouse_position.0.extend(0.0);
    }
}

//  docs: https://bevyengine.org/examples-webgpu/2d-rendering/bounding-2d/
fn display_site_validity(
    mut silhouettes: Query<
        (&mut Sprite, &mut Intersects, &GlobalTransform),
        With<ConstructionSilhouette>,
    >,
    obstacles: Query<(&GlobalTransform, &Aabb), With<Obstacle>>,
) {
    for (mut sprite, mut intersects, silhouette_transform) in silhouettes.iter_mut() {
        let center = silhouette_transform.translation().xy();
        let silhouette_aabb2d = Aabb2d::new(center, SELECTION_SIZE / 2.0);

        let mut hits: bool = false;
        for (obstacle_transform, obstacle_aabb) in obstacles.iter() {
            let obstacle_aabb2d = Aabb2d::new(
                obstacle_transform.translation().xy(),
                obstacle_aabb.half_extents.xy(),
            );
            hits = silhouette_aabb2d.intersects(&obstacle_aabb2d);

            if hits {
                break;
            }
        }

        intersects.0 = hits;

        sprite.color = if hits {
            BUILD_DENIED_COLOR
        } else {
            BUILD_APPROVED_COLOR
        };
    }
}

fn attempt_construction_placement(
    mut placement_attempt: EventReader<AttemptSitePlacement>,
    silhouettes: Query<&Intersects, With<ConstructionSilhouette>>,
    mut energy: ResMut<Energy>,
    build_selection: Res<BuildSelection>,
    mut place_construction_site: EventWriter<PlaceConstructionSite>,
    mut movement_writer: EventWriter<SetUnitPosition>,
) {
    for attempt in placement_attempt.read() {
        for intersects in silhouettes.iter() {
            if !intersects.0 && energy.get(&attempt.team) > build_selection.cost {
                energy.add(&attempt.team, -build_selection.cost);

                place_construction_site.send(PlaceConstructionSite {
                    team: attempt.team.clone(),
                    structure_type: build_selection.structure_type.clone(),
                    position: attempt.position,
                    effort: build_selection.cost,
                });

                //  ensure units move to build
                movement_writer.send(SetUnitPosition {
                    position: attempt.position,
                    direction: Vec2::ONE * (CONSTRUCTION_RANGE - 50.0),
                    formation: Formation::Ringed,
                    team: attempt.team.clone(),
                });
            }
        }
    }
}

fn place_construction_site(
    mut placement_reader: EventReader<PlaceConstructionSite>,
    selected_units: Res<SelectedUnits>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for placement in placement_reader.read() {
        let texture: Handle<Image> = asset_server.load(match placement.structure_type {
            StructureType::SimpleShrine => SIMPLE_SHRINE_ASSET_PATH,
            StructureType::Producer => PRODUCER_ASSET_PATH,
        });

        let pos_3d = placement.position.extend(0.0);

        // info!("assigning workers: {:?}", selected_units.entities.len());
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
            PrimitiveObstacle::Rectangle(Rectangle::from_corners(
                Vec2::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y) / 2.0,
                Vec2::new(SELECTION_SIZE.x, SELECTION_SIZE.y) / 2.0,
            )),
            ConstructionSite {
                structure_type: placement.structure_type.clone(),
                team: placement.team.clone(),
                effort: placement.effort,
                assigned_units: match placement.team {
                    TeamType::Human => selected_units.entities.human.clone(),
                    TeamType::CPU => selected_units.entities.cpu.clone(),
                },
                working_units: Vec::new(),
            },
            Selectable {
                size: SELECTION_SIZE,
            },
            Name::new("ConstructionSite"),
        ));
    }
}

fn assign_new_workers(
    mut assign_workers: EventReader<AssignConstructionWorkers>,
    mut sites: Query<&mut ConstructionSite>,
) {
    for assignment in assign_workers.read() {
        if let Ok(mut site) = sites.get_mut(assignment.site) {
            for unit in assignment.units.iter() {
                site.assigned_units.push(*unit);
            }
        }
    }
}

fn set_assigned_units(
    mut sites: Query<(&Transform, &mut ConstructionSite)>,
    units: Query<&Transform, With<Unit>>,
) {
    for (site_transform, mut site) in sites.iter_mut() {
        let mut swapping: Vec<(usize, Entity)> = vec![];

        //	store all the working units far enough to swap
        for (index, &entity) in site.working_units.iter().enumerate() {
            if let Ok(unit_transform) = units.get(entity) {
                if site_transform
                    .translation
                    .distance(unit_transform.translation)
                    > CONSTRUCTION_RANGE
                {
                    swapping.push((index, entity));
                }
            }
        }

        //	swap the working units to assigned units
        for (index, entity) in swapping.iter() {
            site.assigned_units.push(*entity);
            site.working_units.remove(*index);
        }
    }
}

fn set_working_units(
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
        for (index, entity) in swapping.iter().rev() {
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

fn place_structure(
    sites: Query<(Entity, &GlobalTransform, &ConstructionSite)>,
    mut workers: Query<&mut Idle, With<Unit>>,
    mut build_event: EventWriter<PlaceStructure>,
    mut commands: Commands,
) {
    for (site_entity, transform, site) in sites.iter() {
        if site.effort < 0.0 {
            //  place the structure
            build_event.send(PlaceStructure {
                structure_type: site.structure_type.clone(),
                position: transform.translation(),
                team: site.team.clone(),
            });

            //  set the workers to idle
            // info!("worker count: {:?}", site.working_units.len());
            for &worker_entity in site.working_units.iter() {
                if let Ok(mut idle) = workers.get_mut(worker_entity) {
                    idle.0 = true;
                    // info!("worker {:?} is now idle", worker_entity);
                } else {
                    // info!("worker {:?} has no idle", worker_entity);
                }
            }

            //	destroy the site after building
            commands.entity(site_entity).despawn_recursive();
        }
    }
}
