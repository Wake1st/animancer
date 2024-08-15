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
}

fn set_moveable_location(
    mut reader: EventReader<UnitMovement>,
    mut query: Query<(&mut Moveable, &Selectable), With<Moveable>>,
) {
    for unit_movement in reader.read() {
        let mut order: f32 = 0.0;

        for (mut moveable, selectable) in query.iter_mut() {
            if selectable.selected {
                let (radius, theta) = get_cartesian_position(order);
                order += 1.0;
                moveable.location = vec3(
                    unit_movement.pos.x + radius * f32::cos(theta),
                    unit_movement.pos.y + radius * f32::sin(theta),
                    0.0,
                );
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
fn get_cartesian_position(order: f32) -> (f32, f32) {
    match order {
        1.0..=6.0 => (UNIT_BUFFER, (PI / 3.) * order),
        7.0..=18.0 => (UNIT_BUFFER * 2., (PI / 6.) * order),
        19.0..=42.0 => (UNIT_BUFFER * 3., (PI / 12.) * order),
        43.0..=90.0 => (UNIT_BUFFER * 4., (PI / 24.) * order),
        91.0..=186.0 => (UNIT_BUFFER * 5., (PI / 48.) * order),
        187.0..=378.0 => (UNIT_BUFFER * 6., (PI / 96.) * order),
        379.0..=762.0 => (UNIT_BUFFER * 7., (PI / 192.) * order),
        763.0..=1530.0 => (UNIT_BUFFER * 8., (PI / 384.) * order),
        0.0 => (0.0, 0.0),
        _ => {
            warn!("Unhandled use case - too many units to order in a hexagonal pattern (See 'movement.rs' -> get_cartesian_position).");
            (0.0, 0.0)
        }
    }
}
