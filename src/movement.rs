use std::f32::consts::PI;

use bevy::{math::vec3, prelude::*};

use crate::selectable::Selectable;

const LOCATION_CLOSENESS: f32 = 1.0;
const UNIT_BUFFER: f32 = 40.0;

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
    pub pos: Vec2,
    pub dir: Vec2,
    pub form: Formation,
}

pub enum Formation {
    Ringed,
    Line,
    Box,
    Staggered,
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
            Self::Box => Self::Box,
            Self::Staggered => Self::Staggered,
        }
    }
}

fn set_moveable_location(
    mut reader: EventReader<UnitMovement>,
    mut query: Query<(&mut Moveable, &Selectable), With<Moveable>>,
) {
    for unit_movement in reader.read() {
        let mut order: f32 = 0.0;

        let aim_angle = match unit_movement.dir {
            Vec2::ZERO => 0.0,
            _ => Vec2::X.angle_between(unit_movement.dir),
        };

        for (mut moveable, selectable) in query.iter_mut() {
            if selectable.selected {
                match unit_movement.form {
                    Formation::Ringed => {
                        let (radius, theta) = get_polar_coordinates(order);
                        moveable.location = vec3(
                            unit_movement.pos.x
                                + (radius * UNIT_BUFFER) * f32::cos(theta + aim_angle),
                            unit_movement.pos.y
                                + (radius * UNIT_BUFFER) * f32::sin(theta + aim_angle),
                            0.0,
                        );
                    }
                    Formation::Line => {
                        moveable.location = vec3(
                            unit_movement.pos.x + (order * UNIT_BUFFER) * f32::cos(aim_angle),
                            unit_movement.pos.y + (order * UNIT_BUFFER) * f32::sin(aim_angle),
                            0.0,
                        );
                    }
                    Formation::Box => (),
                    Formation::Staggered => (),
                }
                order += 1.0;
            }
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
/// TODO: move UNIT_BUFFER out of this function - it can be used elsewhere, once
fn get_polar_coordinates(order: f32) -> (f32, f32) {
    match order {
        1.0..=6.0 => (1., (PI / 3.) * order),
        7.0..=18.0 => (2., (PI / 6.) * order),
        19.0..=42.0 => (3., (PI / 12.) * order),
        43.0..=90.0 => (4., (PI / 24.) * order),
        91.0..=186.0 => (5., (PI / 48.) * order),
        187.0..=378.0 => (6., (PI / 96.) * order),
        379.0..=762.0 => (7., (PI / 192.) * order),
        763.0..=1530.0 => (8., (PI / 384.) * order),
        0.0 => (0.0, 0.0),
        _ => {
            warn!("Unhandled use case - too many units to order in a hexagonal pattern (See 'movement.rs' -> get_cartesian_position).");
            (0.0, 0.0)
        }
    }
}
