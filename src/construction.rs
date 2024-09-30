use bevy::{
    math::bounding::{Aabb2d, AabbCast2d, RayCast2d},
    prelude::*,
    render::primitives::Aabb,
};

use crate::{
    inputs::{BuildSelection, MousePosition},
    nav_agent::Obstacle,
    schedule::InGameSet,
    selectable::{Selectable, SelectedUnits, SelectionStateChanged, SelectionType},
    structure::{
        PlaceStructure, StructureType, SELECTION_SIZE, SIMPLE_SHRINE_ASSET_PATH,
        WORKER_PRODUCER_ASSET_PATH,
    },
    unit::Unit,
};

const CONSTRUCTION_BOOST: f32 = 20.5;
pub const CONSTRUCTION_RANGE: f32 = 80.;
const BUILD_APPROVED_COLOR: Color = Color::linear_rgba(0.1, 0.7, 0.0, 0.4);
const BUILD_DENIED_COLOR: Color = Color::linear_rgba(0.7, 0.1, 0.0, 0.4);

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                display_construction_silhouette,
                move_construction_silhouette,
                place_structure,
                assign_new_workers,
                (set_assigned_units, set_working_units).chain(),
                increment_effort,
                place_building,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<PlaceConstructionSite>()
        .add_event::<AssignConstructionWorkers>();
    }
}

#[derive(Component)]
pub struct ConstructionSilhouette {}

#[derive(Component)]
pub struct ConstructionSite {
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
                StructureType::WorkerProducer => WORKER_PRODUCER_ASSET_PATH,
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

//  WIP: https://bevyengine.org/examples-webgpu/2d-rendering/bounding-2d/
fn check_site_validity(
    mut silhouettes: Query<(&mut Sprite, &GlobalTransform), With<ConstructionSilhouette>>,
    obstacles: Query<&GlobalTransform, With<Obstacle>>,
    // mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    for (mut sprite, silhouette_transform) in silhouettes.iter_mut() {
        sprite.color = BUILD_APPROVED_COLOR;

        let silhouette_aabb = Aabb::from_min_max(
            Vec3::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y, 0.0),
            Vec3::new(SELECTION_SIZE.x, SELECTION_SIZE.y, 0.0),
        );

        for obstacle_transform in obstacles.iter() {
            let obstacle_position = obstacle_transform.translation().xy();

            let aabb_ray = Ray2d {
                origin: obstacle_position,
                direction: Dir2::new_unchecked(
                    silhouette_transform.translation().xy() - obstacle_position,
                ),
            };

            let aabb_cast = AabbCast2d {
                aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)),
                ray: RayCast2d::from_ray(aabb_ray, 300.0),
            };

            // for (volume, mut intersects) in volumes.iter_mut() {
            //     let toi = aabb_cast.aabb_collision_at(a);

            //     **intersects = toi.is_some();
            //     if let Some(toi) = toi {
            //         gizmos.rect_2d(
            //             aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * toi,
            //             0.,
            //             aabb_cast.aabb.half_size() * 2.,
            //             LIME,
            //         );
            //     }
            // }
        }
    }
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

        let pos_3d = placement.position.extend(0.0);

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
            Obstacle,
            Aabb::from_min_max(
                Vec3::new(-SELECTION_SIZE.x, -SELECTION_SIZE.y, 0.0),
                Vec3::new(SELECTION_SIZE.x, SELECTION_SIZE.y, 0.0),
            ),
            ConstructionSite {
                structure_type: placement.structure_type.clone(),
                effort: placement.effort,
                assigned_units: selected_units.entities.clone(),
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
