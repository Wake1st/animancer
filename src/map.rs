use bevy::{math::vec2, prelude::*};

use crate::teams::TeamType;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map)
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

fn load_map(mut map: ResMut<Map>) {
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
                segments: 4,
            },
            MapObstacle {
                start: vec2(1200., 1000.),
                end: vec2(1400., 500.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(1400., 300.),
                end: vec2(1400., 0.),
                radius: 1.,
                segments: 1,
            },
            //	SW outer corner
            MapObstacle {
                start: vec2(400., 2200.),
                end: vec2(1200., 2200.),
                radius: 2400.,
                segments: 3,
            },
            MapObstacle {
                start: vec2(1700., 2000.),
                end: vec2(2000.0, 1700.),
                radius: 400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(2200., 1200.),
                end: vec2(2200., 400.),
                radius: 2400.,
                segments: 3,
            },
            //	SE inner corner
            MapObstacle {
                start: vec2(3600., 0.),
                end: vec2(3600., 300.),
                radius: 1.,
                segments: 1,
            },
            MapObstacle {
                start: vec2(3600., 500.),
                end: vec2(3800., 1000.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(4000., 1200.),
                end: vec2(4500., 1400.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(4700., 1400.),
                end: vec2(5000., 1400.),
                radius: 1.,
                segments: 1,
            },
            //	SE outer corner
            MapObstacle {
                start: vec2(2800., 400.),
                end: vec2(2800., 1200.),
                radius: 2400.,
                segments: 3,
            },
            MapObstacle {
                start: vec2(3300., 1700.),
                end: vec2(3000.0, 2000.),
                radius: 400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(3800., 2200.),
                end: vec2(4600., 2200.),
                radius: 2400.,
                segments: 3,
            },
            //	NE inner corner
            MapObstacle {
                start: vec2(5000., 3600.),
                end: vec2(4700., 3600.),
                radius: 1.,
                segments: 1,
            },
            MapObstacle {
                start: vec2(4500., 3600.),
                end: vec2(4000., 3800.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(3800., 4000.),
                end: vec2(3600., 4500.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(3600., 4700.),
                end: vec2(3600., 5000.),
                radius: 1.,
                segments: 1,
            },
            //	NE outer corner
            MapObstacle {
                start: vec2(4600., 2800.),
                end: vec2(3800., 2800.),
                radius: 2400.,
                segments: 3,
            },
            MapObstacle {
                start: vec2(3300., 3000.),
                end: vec2(3000.0, 3300.),
                radius: 400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(2800., 3800.),
                end: vec2(2800., 4600.),
                radius: 2400.,
                segments: 3,
            },
            //	NW inner corner
            MapObstacle {
                start: vec2(1400., 0.),
                end: vec2(1400., 300.),
                radius: 1.,
                segments: 1,
            },
            MapObstacle {
                start: vec2(1400., 500.),
                end: vec2(1200., 1000.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(1000., 1200.),
                end: vec2(500., 1400.),
                radius: 1400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(300., 1400.),
                end: vec2(0., 1400.),
                radius: 1.,
                segments: 1,
            },
            //	NW outer corner
            MapObstacle {
                start: vec2(2200., 4600.),
                end: vec2(2200., 3800.),
                radius: 2400.,
                segments: 3,
            },
            MapObstacle {
                start: vec2(2000., 3300.),
                end: vec2(1700., 3000.),
                radius: 400.,
                segments: 4,
            },
            MapObstacle {
                start: vec2(1200., 2200.),
                end: vec2(400., 2200.),
                radius: 2400.,
                segments: 3,
            },
        ],
        ..default()
    };
}
