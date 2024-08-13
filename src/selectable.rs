use bevy::{math::vec2, prelude::*};

use crate::unit::Unit;

pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_unit, box_select_units))
            .add_event::<UnitSelection>()
            .add_event::<BoxSelection>();
    }
}

#[derive(Component, Default)]
pub struct Selectable {
    pub selected: bool,
}

#[derive(Event)]
pub struct UnitSelection {
    pub pos: Vec2,
}

fn select_unit(
    mut reader: EventReader<UnitSelection>,
    mut query: Query<(&Unit, &GlobalTransform, &mut Selectable), With<Selectable>>,
) {
    for unit_selection in reader.read() {
        let (unit, global_transform, mut selectable) = query.get_single_mut().unwrap();

        let unit_pos = vec2(
            global_transform.translation().x,
            global_transform.translation().y,
        );
        if unit_pos.distance(unit_selection.pos) < unit.size {
            selectable.selected = true;
        }
    }
}

#[derive(Event)]
pub struct BoxSelection {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

fn box_select_units(
    mut reader: EventReader<BoxSelection>,
    mut query: Query<(&GlobalTransform, &mut Selectable), With<Selectable>>,
) {
    for box_selection in reader.read() {
        let (global_transform, mut selectable) = query.get_single_mut().unwrap();

        let unit_pos = vec2(
            global_transform.translation().x,
            global_transform.translation().y,
        );
        if unit_pos.x > box_selection.left
            && unit_pos.x < box_selection.right
            && unit_pos.y > box_selection.bottom
            && unit_pos.y < box_selection.top
        {
            selectable.selected = true;
        }
    }
}
