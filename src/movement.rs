use bevy::{math::vec3, prelude::*};

use crate::selectable::Selectable;

const LOCATION_CLOSENESS: f32 = 1.0;

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
        for (mut moveable, selectable) in query.iter_mut() {
            if selectable.selected {
                moveable.location = vec3(unit_movement.pos.x, unit_movement.pos.y, 0.0);
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
