use std::f32::consts::PI;

use bevy::{
    math::{vec2, VectorSpace},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{teams::TeamType, unit::spawn_hero};

const RESOURCE_SIZE_REDUCER: f32 = 10.0;
const OBSTACLE_WIDTH: f32 = 15.0;

const GROUND_COLOR: Color = Color::linear_rgba(0.0, 0.4, 0.2, 0.1);
const RESOURCES_COLOR: Color = Color::linear_rgba(0.8, 0.0, 0.1, 0.9);
const OBSTACLE_COLOR: Color = Color::linear_rgba(0.4, 0.4, 0.0, 1.0);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (build_map, render_map))
            .insert_resource(Map { ..default() });
    }
}

pub struct PlayerStart {
    pub position: Vec2,
    pub team: TeamType,
}

pub struct MapResource {
    pub position: Vec2,
    pub amount: f32,
}

pub struct MapObstacle {
    pub start: Vec2,
    pub end: Vec2,
    pub radius: f32,
    pub segments: usize,
}

#[derive(Resource)]
pub struct Map {
    pub size: Vec2,
    pub starting_points: Vec<PlayerStart>,
    pub resources: Vec<MapResource>,
    pub obstacles: Vec<MapObstacle>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            size: Default::default(),
            starting_points: Default::default(),
            resources: Default::default(),
            obstacles: Default::default(),
        }
    }
}

fn build_map(mut map: ResMut<Map>) {
    *map = Map {
        size: vec2(5000., 5000.),
        starting_points: vec![
            //	SW corner
            PlayerStart {
                position: vec2(600., 600.),
                team: TeamType::Human,
            },
            //	NE corner
            PlayerStart {
                position: vec2(4400., 4400.),
                team: TeamType::CPU,
            },
        ],
        resources: vec![
            //	corners
            MapResource {
                position: vec2(200., 200.),
                amount: 1200.0,
            },
            MapResource {
                position: vec2(4800., 200.),
                amount: 1200.0,
            },
            MapResource {
                position: vec2(4800., 4800.),
                amount: 1200.0,
            },
            MapResource {
                position: vec2(200., 4800.),
                amount: 1200.0,
            },
            //	middle lanes
            MapResource {
                position: vec2(2500., 600.),
                amount: 800.0,
            },
            MapResource {
                position: vec2(4400., 2500.),
                amount: 800.0,
            },
            MapResource {
                position: vec2(2500., 4400.),
                amount: 800.0,
            },
            MapResource {
                position: vec2(600., 2500.),
                amount: 800.0,
            },
            //	central
            MapResource {
                position: vec2(2500., 2500.),
                amount: 6000.0,
            },
        ],
        obstacles: vec![
            MapObstacle {
                start: vec2(0.,600.),
                end: vec2(600.,0.),
                radius: 600.,
                segments: 12
            },
            //	SW inner corner
            MapObstacle {
                start: vec2(0., 1400.),
                end: vec2(300., 1400.),
                radius: 1.,
                segments: 1,
            },
            MapObstacle {
                start: vec2(500., 1400.),
                end: vec2(1000., 1200.),
                radius: 1400.,
                segments: 12,
            },
            MapObstacle {
                start: vec2(1200., 1000.),
                end: vec2(1400., 500.),
                radius: 1400.,
                segments: 12,
            },
            MapObstacle {
                start: vec2(1400., 300.),
                end: vec2(1400., 0.),
                radius: 1.,
                segments: 1,
            },
            // //	SW outer corner
            // MapObstacle {
            //     start: vec2(400., 2200.),
            //     end: vec2(1200., 2200.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // MapObstacle {
            //     start: vec2(1700., 2000.),
            //     end: vec2(2000.0, 1700.),
            //     radius: 400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(2200., 1200.),
            //     end: vec2(2200., 400.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // //	SE inner corner
            // MapObstacle {
            //     start: vec2(3600., 0.),
            //     end: vec2(3600., 300.),
            //     radius: 1.,
            //     segments: 1,
            // },
            // MapObstacle {
            //     start: vec2(3600., 500.),
            //     end: vec2(3800., 1000.),
            //     radius: 1400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(4000., 1200.),
            //     end: vec2(4500., 1400.),
            //     radius: 1400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(4700., 1400.),
            //     end: vec2(5000., 1400.),
            //     radius: 1.,
            //     segments: 1,
            // },
            // //	SE outer corner
            // MapObstacle {
            //     start: vec2(2800., 400.),
            //     end: vec2(2800., 1200.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // MapObstacle {
            //     start: vec2(3300., 1700.),
            //     end: vec2(3000.0, 2000.),
            //     radius: 400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(3800., 2200.),
            //     end: vec2(4600., 2200.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // //	NE inner corner
            // MapObstacle {
            //     start: vec2(5000., 3600.),
            //     end: vec2(4700., 3600.),
            //     radius: 1.,
            //     segments: 1,
            // },
            // MapObstacle {
            //     start: vec2(4500., 3600.),
            //     end: vec2(4000., 3800.),
            //     radius: 1400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(3800., 4000.),
            //     end: vec2(3600., 4500.),
            //     radius: 1400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(3600., 4700.),
            //     end: vec2(3600., 5000.),
            //     radius: 1.,
            //     segments: 1,
            // },
            // //	NE outer corner
            // MapObstacle {
            //     start: vec2(4600., 2800.),
            //     end: vec2(3800., 2800.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // MapObstacle {
            //     start: vec2(3300., 3000.),
            //     end: vec2(3000.0, 3300.),
            //     radius: 400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(2800., 3800.),
            //     end: vec2(2800., 4600.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // //	NW inner corner
            // MapObstacle {
            //     start: vec2(1400., 0.),
            //     end: vec2(1400., 300.),
            //     radius: 1.,
            //     segments: 1,
            // },
            // MapObstacle {
            //     start: vec2(1400., 500.),
            //     end: vec2(1200., 1000.),
            //     radius: 1400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(1000., 1200.),
            //     end: vec2(500., 1400.),
            //     radius: 1400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(300., 1400.),
            //     end: vec2(0., 1400.),
            //     radius: 1.,
            //     segments: 1,
            // },
            // //	NW outer corner
            // MapObstacle {
            //     start: vec2(2200., 4600.),
            //     end: vec2(2200., 3800.),
            //     radius: 2400.,
            //     segments: 12,
            // },
            // MapObstacle {
            //     start: vec2(2000., 3300.),
            //     end: vec2(1700., 3000.),
            //     radius: 400.,
            //     segments: 16,
            // },
            // MapObstacle {
            //     start: vec2(1200., 2200.),
            //     end: vec2(400., 2200.),
            //     radius: 2400.,
            //     segments: 12,
            // },
        ],
        ..default()
    };
}

fn render_map(
    map: Res<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //	render ground
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(map.size))),
        material: materials.add(GROUND_COLOR),
        transform: Transform::from_translation((map.size / 2.0).extend(-10.0)),
        ..default()
    });

    //	render resources
    for resource in map.resources.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(resource.amount / RESOURCE_SIZE_REDUCER))),
            material: materials.add(RESOURCES_COLOR),
            transform: Transform::from_translation(resource.position.extend(-11.0)),
            ..default()
        });
    }

    //	render obstacles
    for obstacle in map.obstacles.iter() {
        //	get shared values
        let base_vector = obstacle.end - obstacle.start;
        let half_base = base_vector / 2.0;
        let mut transform =
            Transform::from_translation((half_base + obstacle.start).extend(-12.0));
        transform.rotate_z(base_vector.to_angle() - PI / 2.0);
        info!(
            "base vector: {:?}\torigin: {:?}\tangle: {:?}",
            base_vector,
            half_base + obstacle.start,
            base_vector.to_angle() - PI / 2.0
        );

        //	this will need to be somewhat complex
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Capsule2d::new(OBSTACLE_WIDTH, base_vector.length())),
            ),
            material: materials.add(Color::linear_rgba(0.0, 0.0, 0.8, 0.9)),
            transform,
            ..default()
        });
        if obstacle.segments == 1 {
            //  
        } else {
            //	calculate the angles per middle segment
            let end_angle = obstacle.end.to_angle();
            let height = (obstacle.radius * obstacle.radius - half_base.length_squared()).sqrt();
            let origin = obstacle.start + half_base - base_vector.perp().normalize() * height;
            let full_angle = (obstacle.end - origin).angle_between(obstacle.start - origin);
            let segment_angle = full_angle / (obstacle.segments as f32);
            let segment_mid_angle = segment_angle / 2.0;
            let segment_arc = Arc2d::from_radians(obstacle.radius, segment_angle);
            let segment_length = segment_arc.chord_length().abs();

            info!( 
                "\nend_angle: {:?}\nmid: {:?}\nrot-norm: {:?}\nhalf sqrd: {:?}\norigin: {:?}\nfull_angle: {:?}\nsegment_angle: {:?}\nsegment_mid_angle: {:?}\nsegment_arc: {:?}\nsegment_length: {:?}",
                end_angle,obstacle.start + half_base,-base_vector.perp().normalize() * height,half_base.length_squared(),origin,full_angle,segment_angle,segment_mid_angle,segment_arc,segment_length
            );

            for i in 0..obstacle.segments {
                let true_angle = (i as f32 * segment_angle) + segment_mid_angle + end_angle;
                let true_midpoint = Vec2::new(
                    obstacle.radius * f32::cos(true_angle),
                    obstacle.radius * f32::sin(true_angle),
                );
                info!("true mid: {:?}\tplace: {:?}\ttrue angle: {:?}", true_midpoint, true_midpoint + origin, true_angle);
                let mut transform = Transform::from_translation((true_midpoint + origin).extend(-12.0));
                transform.rotate_z(true_angle);

                commands.spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Capsule2d::new(OBSTACLE_WIDTH, segment_length))),
                    material: materials.add(OBSTACLE_COLOR),
                    transform,
                    ..default()
                });
            }
        }
    }

    //  set starting pos
    for point in map.starting_points.iter() {
        spawn_hero(
            &mut commands,
            &asset_server,
            point.position,
            point.team.clone(),
        );
    }
}
