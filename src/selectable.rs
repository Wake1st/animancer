use bevy::{math::vec2, prelude::*};

use crate::unit::Unit;

pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_unit)
            .add_event::<UnitSelection>();
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
