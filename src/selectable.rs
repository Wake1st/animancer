use bevy::{math::vec2, prelude::*};

use crate::{
    construction::{AssignWorkers, ConstructionSite},
    inputs::ProducerSelection,
    structure::Structure,
    unit::{Unit, UnitAction},
};
pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_entities, unit_action_selection))
            .add_event::<BoxSelection>()
            .add_event::<SelectionStateChanged>()
            .insert_resource(SelectedUnits {
                entities: Vec::new(),
            })
            .insert_resource(SelectedStructures {
                entities: Vec::new(),
            })
            .insert_resource(SelectionState(SelectionType::None));
    }
}

#[derive(Component, Default)]
pub struct Selectable {
    pub size: Vec2,
}

#[derive(Event)]
pub struct BoxSelection {
    pub rect: Rect,
}

#[derive(Resource)]
pub struct SelectedUnits {
    pub entities: Vec<Entity>,
}

#[derive(Resource)]
pub struct SelectedStructures {
    pub entities: Vec<Entity>,
}

#[derive(PartialEq)]
pub enum SelectionType {
    None,
    Unit,
    Construction,
    Building,
}

impl Clone for SelectionType {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Unit => Self::Unit,
            Self::Construction => Self::Construction,
            Self::Building => Self::Building,
        }
    }
}

#[derive(Resource)]
pub struct SelectionState(pub SelectionType);

#[derive(Event)]
pub struct SelectionStateChanged {
    pub new_type: SelectionType,
}

fn select_entities(
    mut reader: EventReader<BoxSelection>,
    mut query_units: Query<(Entity, &GlobalTransform, &Selectable), With<Unit>>,
    mut query_structures: Query<(Entity, &GlobalTransform, &Selectable), With<Structure>>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_structures: ResMut<SelectedStructures>,
    mut producer_selection: ResMut<ProducerSelection>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
) {
    for box_selection in reader.read() {
        selected_units.entities.clear();
        selected_structures.entities.clear();

        for (entity, global_transform, selectable) in query_units.iter_mut() {
            //  check if center of unit is within selection box
            //  OR if selection box overlaps unit
            let unit_pos = vec2(
                global_transform.translation().x,
                global_transform.translation().y,
            );
            let unit_rect = Rect::from_center_size(unit_pos, selectable.size);
            if box_selection.rect.contains(unit_rect.center())
                || unit_rect.contains(box_selection.rect.center())
            {
                selected_units.entities.push(entity);
            }
        }

        //  Always prioritize units and never select units AND structures
        if selected_units.entities.len() > 0 {
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::Unit,
            });
            return;
        }

        for (entity, global_transform, selectable) in query_structures.iter_mut() {
            //  check if center of unit is within selection box
            //  OR if selection box overlaps unit
            let structure_pos = vec2(
                global_transform.translation().x,
                global_transform.translation().y,
            );
            let structure_rect = Rect::from_center_size(structure_pos, selectable.size);
            if box_selection.rect.contains(structure_rect.center())
                || structure_rect.contains(box_selection.rect.center())
            {
                selected_structures.entities.push(entity);
            }
        }

        if selected_structures.entities.len() > 0 {
            producer_selection.is_selected = true;
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::Building,
            });
        } else {
            producer_selection.is_selected = false;
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::None,
            });
        }
    }
}

fn unit_action_selection(
    mut unit_action: EventReader<UnitAction>,
    sites: Query<(Entity, &Transform, &Selectable), With<ConstructionSite>>,
    mut assign_worker: EventWriter<AssignWorkers>,
    selected_units: Res<SelectedUnits>,
) {
    for action in unit_action.read() {
        for (entity, transfrom, selectable) in sites.iter() {
            let structure_pos = vec2(transfrom.translation.x, transfrom.translation.y);
            let structure_rect = Rect::from_center_size(structure_pos, selectable.size);
            if structure_rect.contains(action.position) {
                assign_worker.send(AssignWorkers {
                    site: entity,
                    units: selected_units.entities.clone(),
                });
            }
        }
    }
}
