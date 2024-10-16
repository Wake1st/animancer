use bevy::{math::vec2, prelude::*};

use crate::{
    combat::{AssignAttackPursuit, BreakAttackPursuit, Health},
    construction::{AssignConstructionWorkers, ConstructionSite},
    conversion::{AssignConvertPursuit, BreakConvertPursuit, Faith},
    generator::{AssignGeneratorWorkers, Generator},
    inputs::ProducerSelection,
    priest::Priest,
    producer::Producer,
    structure::Structure,
    teams::{Team, TeamBasedValues, TeamType},
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
            entities: Default::default(),
        })
        .insert_resource(SelectedStructures {
            entities: Default::default(),
        })
        .insert_resource(SelectionState(SelectionType::None));
    }
}

#[derive(Component, Default, Debug)]
pub struct Selectable {
    pub size: Vec2,
}

#[derive(Event)]
pub struct BoxSelection {
    pub rect: Rect,
    pub team: TeamType,
}

#[derive(Resource)]
pub struct SelectedUnits {
    pub entities: TeamBasedValues<Entity>,
}

#[derive(Resource)]
pub struct SelectedStructures {
    pub entities: TeamBasedValues<Entity>,
}

#[derive(PartialEq, Debug)]
pub enum SelectionType {
    None,
    Unit,
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
            Self::Unit => Self::Unit,
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
    pub team: TeamType,
}

#[derive(Event)]
struct UnitsSelected {
    pub team: TeamType,
}

#[derive(Event)]
struct StructuresSelected {
    pub team: TeamType,
}

fn select_entities(
    mut reader: EventReader<BoxSelection>,
    mut query_units: Query<(Entity, &Team, &GlobalTransform, &Selectable), With<Unit>>,
    mut query_structures: Query<(Entity, &Team, &GlobalTransform, &Selectable), With<Structure>>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_structures: ResMut<SelectedStructures>,
    mut units_selected: EventWriter<UnitsSelected>,
    mut structures_selected: EventWriter<StructuresSelected>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
    mut producer_selection: ResMut<ProducerSelection>,
) {
    for box_selection in reader.read() {
        // info!(
        //     "selection made for {:?} at {:?}",
        //     box_selection.team, box_selection.rect
        // );

        selected_units.entities.clear(&box_selection.team);
        selected_structures.entities.clear(&box_selection.team);

        for (entity, team, global_transform, selectable) in query_units.iter_mut() {
            // info!(
            //     "\nunit query ->\n\tentity: {:?}\n\tteam: {:?}\n\tglobal_transform: {:?}\n\tselectable: {:?}",
            //     entity, team, global_transform, selectable
            // );
            //  ensure the player's units are selected
            if team.0 != box_selection.team {
                continue;
            }

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
                selected_units.entities.push(&box_selection.team, entity);
            }
        }

        // info!(
        //     "selected units: {:?}",
        //     selected_units.entities.len(&box_selection.team)
        // );
        //  Always prioritize units and never select units AND structures
        if selected_units.entities.len(&box_selection.team) > 0 {
            units_selected.send(UnitsSelected {
                team: box_selection.team.clone(),
            });
            return;
        }

        for (entity, team, global_transform, selectable) in query_structures.iter_mut() {
            // info!(
            //     "\nstructure query ->\n\tentity: {:?}\n\tteam: {:?}\n\tglobal_transform: {:?}\n\tselectable: {:?}",
            //     entity, team, global_transform, selectable
            // );
            //  ensure the player's units are selected
            if team.0 != box_selection.team {
                continue;
            }

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
                selected_structures
                    .entities
                    .push(&box_selection.team, entity);
            }
        }

        // info!(
        //     "selected structures: {:?}",
        //     selected_structures.entities.len(&box_selection.team)
        // );
        if selected_structures.entities.len(&box_selection.team) > 0 {
            structures_selected.send(StructuresSelected {
                team: box_selection.team.clone(),
            });
        } else {
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::None,
                team: box_selection.team.clone(),
            });

            producer_selection.is_selected = false;
        }
    }
}

fn set_selected_unit_type(
    mut selection_event: EventReader<UnitsSelected>,
    selected_units: Res<SelectedUnits>,
    worker_query: Query<Entity, With<Worker>>,
    priest_query: Query<Entity, With<Priest>>,
    warrior_query: Query<Entity, With<Warrior>>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
) {
    for units_selected in selection_event.read() {
        let mut selected_type = SelectionType::None;
        let mut mismatched_types: bool = false;

        // info!("unit selection team: {:?}", units_selected.team);
        for &entity in selected_units.entities.iter(&units_selected.team) {
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
        // info!("selection type: {:?}", selected_type);

        if mismatched_types {
            selection_state_changed.send(SelectionStateChanged {
                new_type: SelectionType::Unit,
                team: units_selected.team.clone(),
            });
        } else {
            selection_state_changed.send(SelectionStateChanged {
                new_type: selected_type,
                team: units_selected.team.clone(),
            });
        }
    }
}

fn set_selected_structure_type(
    mut selection_event: EventReader<StructuresSelected>,
    selected_structures: Res<SelectedStructures>,
    generator_query: Query<Entity, With<Generator>>,
    producer_query: Query<Entity, With<Producer>>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
    mut producer_selection: ResMut<ProducerSelection>,
) {
    for structures_selected in selection_event.read() {
        let mut selected_type = SelectionType::None;
        let mut mismatched_types: bool = false;

        for &entity in selected_structures.entities.iter(&structures_selected.team) {
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
                team: structures_selected.team.clone(),
            });
        } else {
            selection_state_changed.send(SelectionStateChanged {
                new_type: selected_type.clone(),
                team: structures_selected.team.clone(),
            });

            if structures_selected.team == TeamType::Human {
                producer_selection.is_selected = selected_type == SelectionType::Producer;
            }
        }
    }
}

fn unit_action_selection(
    mut unit_action: EventReader<UnitAction>,
    sites: Query<(Entity, &Team, &Transform, &Selectable), With<ConstructionSite>>,
    mut assign_construction_worker: EventWriter<AssignConstructionWorkers>,
    generators: Query<(Entity, &Team, &Transform, &Selectable), With<Generator>>,
    mut assign_generator_workers: EventWriter<AssignGeneratorWorkers>,
    attackables: Query<(Entity, &Team, &Transform, &Selectable), With<Health>>,
    mut assign_attack_pursuit: EventWriter<AssignAttackPursuit>,
    mut break_attack_pursuit: EventWriter<BreakAttackPursuit>,
    convertables: Query<(Entity, &Team, &Transform, &Selectable), With<Faith>>,
    mut assign_convert_pursuit: EventWriter<AssignConvertPursuit>,
    mut break_convert_pursuit: EventWriter<BreakConvertPursuit>,
    selected_units: Res<SelectedUnits>,
) {
    for action in unit_action.read() {
        for (entity, team, transform, selectable) in sites.iter() {
            //  ensure only the players buildings are selected
            if team.0 != TeamType::Human {
                continue;
            }

            let structure_pos = vec2(transform.translation.x, transform.translation.y);
            let structure_rect = Rect::from_center_size(structure_pos, selectable.size);
            if structure_rect.contains(action.position) {
                assign_construction_worker.send(AssignConstructionWorkers {
                    site: entity,
                    units: selected_units.entities.human.clone(),
                });
            }
        }

        for (entity, team, transform, selectable) in generators.iter() {
            //  ensure only the players buildings are selected
            if team.0 != TeamType::Human {
                continue;
            }

            let structure_pos = vec2(transform.translation.x, transform.translation.y);
            let structure_rect = Rect::from_center_size(structure_pos, selectable.size);
            if structure_rect.contains(action.position) {
                assign_generator_workers.send(AssignGeneratorWorkers {
                    generator: entity,
                    workers: selected_units.entities.human.clone(),
                });
            }
        }

        for (entity, team, transform, selectable) in attackables.iter() {
            //  ensure only enemies are selected
            if team.0 == TeamType::Human {
                continue;
            }

            let unit_pos = vec2(transform.translation.x, transform.translation.y);
            let unit_rect = Rect::from_center_size(unit_pos, selectable.size);
            if unit_rect.contains(action.position) {
                assign_attack_pursuit.send(AssignAttackPursuit {
                    predators: selected_units.entities.human.clone(),
                    prey: entity,
                });
            } else {
                break_attack_pursuit.send(BreakAttackPursuit {
                    entities: selected_units.entities.human.clone(),
                });
            }
        }

        for (entity, team, transform, selectable) in convertables.iter() {
            //  ensure only enemies are selected
            if team.0 == TeamType::Human {
                continue;
            }

            let unit_pos = vec2(transform.translation.x, transform.translation.y);
            let unit_rect = Rect::from_center_size(unit_pos, selectable.size);
            if unit_rect.contains(action.position) {
                assign_convert_pursuit.send(AssignConvertPursuit {
                    predators: selected_units.entities.human.clone(),
                    prey: entity,
                });
            } else {
                break_convert_pursuit.send(BreakConvertPursuit {
                    entities: selected_units.entities.human.clone(),
                });
            }
        }
    }
}
