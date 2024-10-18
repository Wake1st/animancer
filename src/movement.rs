use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{nav_agent::AssignNavigatorPath, selectable::SelectedUnits, teams::TeamType};

const UNIT_BUFFER: f32 = 40.0;
const LINE_STRENGTH_SCALE: f32 = 2.4;
const RINGED_STRENGTH_SCALE: f32 = 0.9;
const BOX_STRENGTH_SCALE: f32 = 0.9;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_moveable_location,
                // move_unit
            ),
        )
        .add_event::<SetUnitPosition>();
    }
}

#[derive(Component)]
pub struct Moving(pub bool);

#[derive(Component, Default)]
pub struct Moveable {
    pub location: Vec3,
}

#[derive(Event, Default)]
pub struct SetUnitPosition {
    pub position: Vec2,
    pub direction: Vec2,
    pub formation: Formation,
    pub team: TeamType,
}

pub enum Formation {
    Ringed,
    Line,
    Box,
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
        }
    }
}

fn set_moveable_location(
    mut reader: EventReader<SetUnitPosition>,
    mut query: Query<&mut Moveable>,
    selected: Res<SelectedUnits>,
    mut nav_path_assigner: EventWriter<AssignNavigatorPath>,
) {
    for unit_movement in reader.read() {
        let unit_count = selected.entities.len(&unit_movement.team) as f32;
        let (aim_angle, strength) = match unit_movement.direction {
            Vec2::ZERO => (0.0, UNIT_BUFFER),
            d => (Vec2::X.angle_between(d), UNIT_BUFFER + d.length()),
        };

        match unit_movement.formation {
            Formation::Ringed => {
                let mut order: f32 = 0.0;
                let radius = RINGED_STRENGTH_SCALE * strength;
                let circumference = 2. * PI * radius;
                let theta = (circumference / unit_count) / radius;

                for &entity in selected.entities.iter(&unit_movement.team) {
                    if let Ok(mut moveable) = query.get_mut(entity) {
                        moveable.location = vec3(
                            unit_movement.position.x + radius * f32::cos(order * theta + aim_angle),
                            unit_movement.position.y + radius * f32::sin(order * theta + aim_angle),
                            0.0,
                        );

                        nav_path_assigner.send(AssignNavigatorPath {
                            entity,
                            location: moveable.location,
                        });
                    }
                    order += 1.0;
                }
            }
            Formation::Line => {
                let mut order: f32 = 0.0;
                let line_count =
                    f32::ceil(LINE_STRENGTH_SCALE * strength / (unit_count * UNIT_BUFFER));
                let units_per_line = f32::ceil(unit_count / line_count);

                for &entity in selected.entities.iter(&unit_movement.team) {
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

                        nav_path_assigner.send(AssignNavigatorPath {
                            entity,
                            location: moveable.location,
                        });
                    }
                    order += 1.0;
                }
            }
            Formation::Box => {
                let mut distance_traveled = 0.0;
                let mut line_count = 1.0;
                let half_side = BOX_STRENGTH_SCALE * strength;
                let side_length = half_side * 2.0;
                let total_length = side_length * 4.0;
                let unit_spacing = total_length / unit_count;

                //  move along the square, placing units evenly apart
                for &entity in selected.entities.iter(&unit_movement.team) {
                    if let Ok(mut moveable) = query.get_mut(entity) {
                        distance_traveled += unit_spacing;
                        if distance_traveled / (line_count * side_length) > 1.0 {
                            line_count += 1.0;
                        }
                        let line_distance = distance_traveled - side_length * (line_count - 1.0);

                        //  move along proper line
                        let position = match line_count {
                            1.0 => vec2(line_distance - half_side, half_side),
                            2.0 => vec2(half_side, half_side - line_distance),
                            3.0 => vec2(half_side - line_distance, -half_side),
                            4.0..=4.1 => vec2(-half_side, line_distance - half_side),
                            _ => vec2(0., 0.),
                        };

                        moveable.location = vec3(
                            unit_movement.position.x
                                + position.length()
                                    * f32::cos(aim_angle - PI / 2. + position.to_angle()),
                            unit_movement.position.y
                                + position.length()
                                    * f32::sin(aim_angle - PI / 2. + position.to_angle()),
                            0.0,
                        );

                        nav_path_assigner.send(AssignNavigatorPath {
                            entity,
                            location: moveable.location,
                        });
                    }
                }
            }
        }
    }
}
