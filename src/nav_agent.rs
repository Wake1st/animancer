use bevy::{math::vec2, prelude::*};
use vleue_navigator::prelude::*;

use crate::schedule::InGameSet;

const MESH_WIDTH: u32 = 1680;
const MESH_HEIGHT: u32 = 840;

pub struct NavAgentPlugin;

impl Plugin for NavAgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_navmesh)
            .add_systems(
                Update,
                (
                    give_target_to_navigator::<10, MESH_WIDTH, MESH_HEIGHT>,
                    move_navigator,
                    refresh_path::<10, MESH_WIDTH, MESH_HEIGHT>,
                )
                    .in_set(InGameSet::EntityUpdates),
            )
            .add_event::<AssignNavigatorPath>();
    }
}

#[derive(Component, Debug)]
pub struct Obstacle;

#[derive(Component)]
pub struct Navigator {
    pub speed: f32,
}

#[derive(Component)]
pub struct Path {
    pub current: Vec2,
    pub next: Vec<Vec2>,
    pub target: Entity,
}

#[derive(Event)]
pub struct AssignNavigatorPath {
    pub entity: Entity,
    pub location: Vec3,
}

fn spawn_navmesh(mut commands: Commands) {
    commands.spawn(NavMeshBundle {
        settings: NavMeshSettings {
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                vec2(0.0, 0.0),
                vec2(MESH_WIDTH as f32, 0.0),
                vec2(MESH_WIDTH as f32, MESH_HEIGHT as f32),
                vec2(0.0, MESH_HEIGHT as f32),
            ]),
            ..default()
        },
        // Mark it for update as soon as obstacles are changed.
        // Other modes can be debounced or manually triggered.
        update_mode: NavMeshUpdateMode::Direct,
        transform: Transform::from_translation(Vec3::new(
            -(MESH_WIDTH as f32) / 2.0,
            -(MESH_HEIGHT as f32) / 2.0,
            0.0,
        )),
        ..NavMeshBundle::with_default_id()
    });
}

fn give_target_to_navigator<const SIZE: u32, const X: u32, const Y: u32>(
    mut nav_path_assignment: EventReader<AssignNavigatorPath>,
    navigator: Query<&Transform, With<Navigator>>,
    navmeshes: Res<Assets<NavMesh>>,
    navmesh: Query<&Handle<NavMesh>>,
    mut commands: Commands,
) {
    for assignment in nav_path_assignment.read() {
        for transform in &navigator.get(assignment.entity) {
            let Some(navmesh) = navmeshes.get(navmesh.single()) else {
                continue;
            };
            let position = assignment.location
                + Vec3::new((MESH_WIDTH as f32) / 2.0, (MESH_HEIGHT as f32) / 2.0, 0.0);

            //	Check if movement position is in mesh
            info!("checking position");
            if !navmesh.is_in_mesh(position.xy()) {
                continue;
            }
            info!("is in mesh");

            //	Create a path that the mesh understands
            let Some(path) = navmesh.transformed_path(
                transform.translation.xyz(),
                navmesh.transform().transform_point(position),
            ) else {
                //	we need some way of handling the no-path scenario
                break;
            };

            //	Setting the path
            if let Some((first, remaining)) = path.path.split_first() {
                let mut remaining = remaining.iter().map(|p| p.xy()).collect::<Vec<_>>();

                remaining.reverse();
                let id = commands
                    .spawn(TransformBundle {
                        local: Transform::from_translation(
                            remaining.first().unwrap_or(&first.xy()).extend(1.5),
                        ),
                        ..default()
                    })
                    .id();
                commands.entity(assignment.entity).insert(Path {
                    current: first.xy(),
                    next: remaining,
                    target: id,
                });
            }
        }
    }
}

fn refresh_path<const SIZE: u32, const X: u32, const Y: u32>(
    mut commands: Commands,
    mut navigator: Query<(Entity, &Transform, &mut Path), With<Navigator>>,
    mut navmeshes: ResMut<Assets<NavMesh>>,
    navmesh: Query<(&Handle<NavMesh>, Ref<NavMeshStatus>)>,
    transforms: Query<&Transform>,
    mut delta: Local<f32>,
) {
    let (navmesh_handle, status) = navmesh.single();
    if (!status.is_changed() || *status != NavMeshStatus::Built) && *delta == 0.0 {
        return;
    }
    let Some(navmesh) = navmeshes.get_mut(navmesh_handle) else {
        return;
    };

    for (entity, transform, mut path) in &mut navigator {
        let target = transforms.get(path.target).unwrap().translation.xy();
        if !navmesh.transformed_is_in_mesh(transform.translation) {
            *delta += 0.1;
            navmesh.set_search_delta(*delta);
            continue;
        }
        if !navmesh.transformed_is_in_mesh(target.extend(0.0)) {
            commands.entity(path.target).despawn_recursive();
            commands.entity(entity).remove::<Path>();
            continue;
        }

        let Some(new_path) = navmesh.transformed_path(transform.translation, target.extend(0.0))
        else {
            commands.entity(path.target).despawn_recursive();
            commands.entity(entity).remove::<Path>();
            continue;
        };
        if let Some((first, remaining)) = new_path.path.split_first() {
            let mut remaining = remaining.iter().map(|p| p.xy()).collect::<Vec<_>>();
            remaining.reverse();
            path.current = first.xy();
            path.next = remaining;
            *delta = 0.0;
        }
    }
}

fn move_navigator(
    mut commands: Commands,
    mut navigator: Query<(&mut Transform, &mut Path, Entity, &Navigator)>,
    time: Res<Time>,
) {
    for (mut transform, mut path, entity, navigator) in navigator.iter_mut() {
        let move_direction = path.current - transform.translation.xy();
        transform.translation +=
            (move_direction.normalize() * time.delta_seconds() * navigator.speed).extend(0.0);
        while transform.translation.xy().distance(path.current) < navigator.speed / 50.0 {
            if let Some(next) = path.next.pop() {
                path.current = next;
            } else {
                commands.entity(path.target).despawn_recursive();
                commands.entity(entity).remove::<Path>();
                break;
            }
        }
    }
}
