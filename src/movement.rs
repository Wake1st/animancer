use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::selectable::SelectedUnits;

const LOCATION_CLOSENESS: f32 = 1.0;
const UNIT_BUFFER: f32 = 40.0;
const LINE_STRENGTH_SCALE: f32 = 2.4;
const RINGED_STRENGTH_SCALE: f32 = 0.4;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_moveable_location, move_unit))
            .add_event::<UnitMovement>();
    }
}

#[derive(Component, Default)]
pub struct Moveable {
    pub speed: f32,
    pub location: Vec3,
}

#[derive(Event)]
pub struct UnitMovement {
    pub position: Vec2,
    pub direction: Vec2,
    pub formation: Formation,
}

pub enum Formation {
    Ringed,
    Line,
    // Box,
    // Staggered,
}

impl Default for Formation {
    fn default() -> Self {
        Formation::Ringed
    }
}

impl Clone for Formation {
    fn clone(&self) -> Self {
        match self {
            Self::Ringed => Self::Ringed,
            Self::Line => Self::Line,
        }
    }
}

fn set_moveable_location(
    mut reader: EventReader<UnitMovement>,
    mut query: Query<&mut Moveable>,
    selected: Res<SelectedUnits>,
) {
    for unit_movement in reader.read() {
        //  TODO: store window size (see example: https://bevyengine.org/examples-webgpu/3d-rendering/split-screen/)
        let unit_count = selected.entities.len() as f32;
        let (aim_angle, strength) = match unit_movement.direction {
            Vec2::ZERO => (0.0, UNIT_BUFFER),
            d => (Vec2::X.angle_between(d), UNIT_BUFFER + d.length()),
        };

        match unit_movement.formation {
            Formation::Ringed => {
                let mut order: f32 = 0.0;
                for &entity in selected.entities.iter() {
                    if let Ok(mut moveable) = query.get_mut(entity) {
                        let (radius, theta) = get_polar_coordinates(order, unit_count);
                        moveable.location = vec3(
                            unit_movement.position.x
                                + (radius
                                    * RINGED_STRENGTH_SCALE
                                    * strength
                                    * f32::cos(theta + aim_angle)),
                            unit_movement.position.y
                                + (radius
                                    * RINGED_STRENGTH_SCALE
                                    * strength
                                    * f32::sin(theta + aim_angle)),
                            0.0,
                        );
                    }
                    order += 1.0;
                }
            }
            Formation::Line => {
                let line_count =
                    f32::ceil(LINE_STRENGTH_SCALE * strength / (unit_count * UNIT_BUFFER));
                let units_per_line = f32::ceil(unit_count / line_count);

                let mut order: f32 = 0.0;
                for &entity in selected.entities.iter() {
                    if let Ok(mut moveable) = query.get_mut(entity) {
                        let current_line_index = f32::floor(order / units_per_line);
                        let stagger =
                            if line_count == 1.0 || (current_line_index + 1.0) != line_count {
                                0.5
                            } else {
                                1.0
                            };

                        let position = vec2(
                            (order % units_per_line) - units_per_line / 2. + stagger,
                            -f32::floor(order / units_per_line),
                        ) * UNIT_BUFFER;

                        moveable.location = vec3(
                            unit_movement.position.x
                                + position.length()
                                    * f32::cos(aim_angle - PI / 2. + position.to_angle()),
                            unit_movement.position.y
                                + position.length()
                                    * f32::sin(aim_angle - PI / 2. + position.to_angle()),
                            0.0,
                        );
                    }
                    order += 1.0;
                }
            } // Formation::Box => (),
              // Formation::Staggered => (),
        }
    }
}

fn move_unit(mut query: Query<(&mut Transform, &Moveable)>, time: Res<Time>) {
    for (mut transform, moveable) in query.iter_mut() {
        if transform.translation.distance(moveable.location) > LOCATION_CLOSENESS {
            let direction = (moveable.location - transform.translation).normalize();
            transform.translation += direction * moveable.speed * time.delta_seconds();
        };
    }
}

/// The units are spaced in an exact hexagonal pattern
/// TODO: it might be better to space them evenly as a group (which would have non-hex-based layers)
fn get_polar_coordinates(order: f32, unit_count: f32) -> (f32, f32) {
    match order {
        1.0..=6.0 => (
            1.,
            (if unit_count >= 6.0 {
                PI / 3.
            } else {
                2. * PI / (unit_count - 1.0)
            }) * order,
        ),
        7.0..=18.0 => (
            2.,
            (if unit_count >= 18.0 {
                PI / 18.0
            } else {
                2. * PI / (unit_count - 7.0)
            }) * order,
        ),
        19.0..=42.0 => (
            3.,
            (if unit_count >= 42.0 {
                PI / 42.0
            } else {
                2. * PI / (unit_count - 19.0)
            }) * order,
        ),
        43.0..=90.0 => (
            4.,
            (if unit_count >= 90.0 {
                PI / 90.0
            } else {
                2. * PI / (unit_count - 43.0)
            }) * order,
        ),
        91.0..=186.0 => (
            5.,
            (if unit_count >= 186.0 {
                PI / 186.0
            } else {
                2. * PI / (unit_count - 91.0)
            }) * order,
        ),
        187.0..=378.0 => (
            6.,
            (if unit_count >= 378.0 {
                PI / 378.0
            } else {
                2. * PI / (unit_count - 187.0)
            }) * order,
        ),
        379.0..=762.0 => (
            7.,
            (if unit_count >= 762.0 {
                PI / 762.0
            } else {
                2. * PI / (unit_count - 379.0)
            }) * order,
        ),
        763.0..=1530.0 => (
            8.,
            (if unit_count >= 1530.0 {
                PI / 1530.0
            } else {
                2. * PI / (unit_count - 763.0)
            }) * order,
        ),
        0.0 => (0.0, 0.0),
        _ => {
            warn!("Unhandled use case - too many units to order in a hexagonal pattern (See 'movement.rs' -> get_cartesian_position).");
            (0.0, 0.0)
        }
    }
}
