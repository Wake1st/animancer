use bevy::prelude::*;

use crate::{
    combat::AttackPursuit,
    conversion::ConvertPursuit,
    selectable::{SelectedStructures, SelectedUnits},
    structure::Structure,
    unit::Unit,
};

const SELECTED_UNIT_RADIUS: f32 = 20.0;
const ATTACKED_UNIT_RADIUS: f32 = 22.0;
const CONVERTING_UNIT_RADIUS: f32 = 24.0;
const SELECTED_STRUCTURE_RADIUS: f32 = 36.0;
const ATTACKED_STRUCTURE_RADIUS: f32 = 38.0;

const SELECTION_COLOR: Color = Color::linear_rgba(0.4, 0.4, 0.0, 0.4);
const ATTACKED_COLOR: Color = Color::linear_rgba(0.7, 0.0, 0.2, 0.4);
const CONVERTING_COLOR: Color = Color::linear_rgba(0.0, 0.3, 0.8, 0.4);

pub struct VisualFeedbackPlugin;

impl Plugin for VisualFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                highlight_selected_units,
                highlight_attacked_units,
                highlight_converting_units,
                highlight_selected_structures,
                highlight_attacked_structures,
            ),
        );
    }
}

fn highlight_selected_units(
    selected_units: Res<SelectedUnits>,
    units: Query<&Transform, With<Unit>>,
    mut gizmos: Gizmos,
) {
    for &entity in selected_units.entities.iter() {
        if let Ok(transform) = units.get(entity) {
            gizmos.circle_2d(
                transform.translation.xy(),
                SELECTED_UNIT_RADIUS,
                SELECTION_COLOR,
            );
        }
    }
}

fn highlight_attacked_units(
    attackers: Query<&AttackPursuit>,
    prey: Query<&Transform, With<Unit>>,
    mut gizmos: Gizmos,
) {
    for attacker in attackers.iter() {
        if let Ok(transform) = prey.get(attacker.prey) {
            gizmos.circle_2d(
                transform.translation.xy(),
                ATTACKED_UNIT_RADIUS,
                ATTACKED_COLOR,
            );
        }
    }
}

fn highlight_converting_units(
    converters: Query<&ConvertPursuit>,
    prey: Query<&Transform, With<Unit>>,
    mut gizmos: Gizmos,
) {
    for converter in converters.iter() {
        if let Ok(transform) = prey.get(converter.prey) {
            gizmos.circle_2d(
                transform.translation.xy(),
                CONVERTING_UNIT_RADIUS,
                CONVERTING_COLOR,
            );
        }
    }
}

fn highlight_selected_structures(
    selected_structures: Res<SelectedStructures>,
    structures: Query<&Transform, With<Structure>>,
    mut gizmos: Gizmos,
) {
    for &entity in selected_structures.entities.iter() {
        if let Ok(transform) = structures.get(entity) {
            gizmos.circle_2d(
                transform.translation.xy(),
                SELECTED_STRUCTURE_RADIUS,
                SELECTION_COLOR,
            );
        }
    }
}

fn highlight_attacked_structures(
    attackers: Query<&AttackPursuit>,
    prey: Query<&Transform, With<Structure>>,
    mut gizmos: Gizmos,
) {
    for attacker in attackers.iter() {
        if let Ok(transform) = prey.get(attacker.prey) {
            gizmos.circle_2d(
                transform.translation.xy(),
                ATTACKED_STRUCTURE_RADIUS,
                ATTACKED_COLOR,
            );
        }
    }
}
