use bevy::{math::vec2, prelude::*};

pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_units)
            .add_event::<BoxSelection>();
    }
}

#[derive(Component, Default)]
pub struct Selectable {
    pub selected: bool,
    pub size: Vec2,
}

#[derive(Event)]
pub struct BoxSelection {
    pub rect: Rect,
}

fn select_units(
    mut reader: EventReader<BoxSelection>,
    mut query: Query<(&GlobalTransform, &mut Selectable), With<Selectable>>,
) {
    for box_selection in reader.read() {
        for (global_transform, mut selectable) in query.iter_mut() {
            //  check if center of unit is within selection box
            //  OR if selection box overlaps unit
            let unit_pos = vec2(
                global_transform.translation().x,
                global_transform.translation().y,
            );
            let unit_rect = Rect::from_center_size(unit_pos, selectable.size);
            selectable.selected = box_selection.rect.contains(unit_rect.center())
                || unit_rect.contains(box_selection.rect.center());
        }
    }
}
