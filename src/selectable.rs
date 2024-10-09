use bevy::{math::vec2, prelude::*};

use crate::{
    combat::{AssignAttackPursuit, Health},
    construction::{AssignConstructionWorkers, ConstructionSite},
    generator::{AssignGeneratorWorkers, Generator},
    inputs::ProducerSelection,
    priest::Priest,
    producer::Producer,
    structure::Structure,
    unit::{Unit, UnitAction},
    warrior::Warrior,
    worker::Worker,
};
pub struct SelectablePlugin;

impl Plugin for SelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    select_entities,
                    (set_selected_unit_type, set_selected_structure_type),
                )
                    .chain(),
                unit_action_selection,
            ),
        )
        .add_event::<BoxSelection>()
        .add_event::<SelectionStateChanged>()
        .add_event::<UnitsSelected>()
        .add_event::<StructuresSelected>()
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

#[derive(PartialEq, Debug)]
pub enum SelectionType {
    None,
    Worker,
    Priest,
    Warrior,
    Construction,
    Generator,
    Producer,
}

impl Clone for SelectionType {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Worker => Self::Worker,
            Self::Priest => Self::Priest,
            Self::Warrior => Self::Warrior,
            Self::Construction => Self::Construction,
            Self::Generator => Self::Generator,
            Self::Producer => Self::Producer,
        }
    }
}

#[derive(Resource)]
pub struct SelectionState(pub SelectionType);

#[derive(Event)]
pub struct SelectionStateChanged {
    pub new_type: SelectionType,
}

#[derive(Event)]
struct UnitsSelected {}

#[derive(Event)]
struct StructuresSelected {}

fn select_entities(
    mut reader: EventReader<BoxSelection>,
    mut query_units: Query<(Entity, &GlobalTransform, &Selectable), With<Unit>>,
    mut query_structures: Query<(Entity, &GlobalTransform, &Selectable), With<Structure>>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_structures: ResMut<SelectedStructures>,
    mut units_selected: EventWriter<UnitsSelected>,
    mut structures_selected: EventWriter<StructuresSelected>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
    mut producer_selection: ResMut<ProducerSelection>,
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
            units_selected.send(UnitsSelected {});
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
            structures_selected.send(StructuresSelected {});
        } else {
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::None,
            });

            producer_selection.is_selected = false;
        }
    }
}

fn set_selected_unit_type(
    mut units_selected: EventReader<UnitsSelected>,
    selected_units: Res<SelectedUnits>,
    worker_query: Query<Entity, With<Worker>>,
    priest_query: Query<Entity, With<Priest>>,
    warrior_query: Query<Entity, With<Warrior>>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
) {
    for _ in units_selected.read() {
        let mut selected_type = SelectionType::None;
        let mut mismatched_types: bool = false;

        for &entity in selected_units.entities.iter() {
            if let Ok(_) = worker_query.get(entity) {
                if selected_type == SelectionType::None {
                    selected_type = SelectionType::Worker;
                    continue;
                } else if selected_type != SelectionType::Worker {
                    mismatched_types = true;
                    break;
                }
            }
            if let Ok(_) = priest_query.get(entity) {
                if selected_type == SelectionType::None {
                    selected_type = SelectionType::Priest;
                    continue;
                } else if selected_type != SelectionType::Priest {
                    mismatched_types = true;
                    break;
                }
            }
            if let Ok(_) = warrior_query.get(entity) {
                if selected_type == SelectionType::None {
                    selected_type = SelectionType::Warrior;
                    continue;
                } else if selected_type != SelectionType::Warrior {
                    mismatched_types = true;
                    break;
                }
            }
        }

        if mismatched_types {
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::None,
            });
        } else {
            selection_state_changed.send(SelectionStateChanged {
                new_type: selected_type,
            });
        }
    }
}

fn set_selected_structure_type(
    mut structures_selected: EventReader<StructuresSelected>,
    selected_structures: Res<SelectedStructures>,
    generator_query: Query<Entity, With<Generator>>,
    producer_query: Query<Entity, With<Producer>>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
    mut producer_selection: ResMut<ProducerSelection>,
) {
    for _ in structures_selected.read() {
        let mut selected_type = SelectionType::None;
        let mut mismatched_types: bool = false;

        for &entity in selected_structures.entities.iter() {
            if let Ok(_) = generator_query.get(entity) {
                if selected_type == SelectionType::None {
                    selected_type = SelectionType::Generator;
                    continue;
                } else if selected_type != SelectionType::Generator {
                    mismatched_types = true;
                    break;
                }
            }
            if let Ok(_) = producer_query.get(entity) {
                if selected_type == SelectionType::None {
                    selected_type = SelectionType::Producer;
                    continue;
                } else if selected_type != SelectionType::Producer {
                    mismatched_types = true;
                    break;
                }
            }
        }

        if mismatched_types {
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::None,
            });
        } else {
            selection_state_changed.send(SelectionStateChanged {
                new_type: selected_type.clone(),
            });

            producer_selection.is_selected = selected_type == SelectionType::Producer;
        }
    }
}

fn unit_action_selection(
    mut unit_action: EventReader<UnitAction>,
    sites: Query<(Entity, &Transform, &Selectable), With<ConstructionSite>>,
    mut assign_construction_worker: EventWriter<AssignConstructionWorkers>,
    generators: Query<(Entity, &Transform, &Selectable), With<Generator>>,
    mut assign_generator_workers: EventWriter<AssignGeneratorWorkers>,
    attackables: Query<(Entity, &Transform, &Selectable), With<Health>>,
    mut assign_attack_pursuit: EventWriter<AssignAttackPursuit>,
    selected_units: Res<SelectedUnits>,
) {
    for action in unit_action.read() {
        for (entity, transform, selectable) in sites.iter() {
            let structure_pos = vec2(transform.translation.x, transform.translation.y);
            let structure_rect = Rect::from_center_size(structure_pos, selectable.size);
            if structure_rect.contains(action.position) {
                assign_construction_worker.send(AssignConstructionWorkers {
                    site: entity,
                    units: selected_units.entities.clone(),
                });
            }
        }

        for (entity, transform, selectable) in generators.iter() {
            let structure_pos = vec2(transform.translation.x, transform.translation.y);
            let structure_rect = Rect::from_center_size(structure_pos, selectable.size);
            if structure_rect.contains(action.position) {
                assign_generator_workers.send(AssignGeneratorWorkers {
                    generator: entity,
                    workers: selected_units.entities.clone(),
                });
            }
        }

        for (entity, transform, selectable) in attackables.iter() {
            let unit_pos = vec2(transform.translation.x, transform.translation.y);
            let unit_rect = Rect::from_center_size(unit_pos, selectable.size);
            if unit_rect.contains(action.position) {
                assign_attack_pursuit.send(AssignAttackPursuit {
                    predators: selected_units.entities.clone(),
                    prey: entity,
                });
            }
        }
    }
}
