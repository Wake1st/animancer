use bevy::prelude::*;

use crate::{
    movement::{Formation, SetUnitPosition},
    priest::Priest,
    teams::{Team, TeamType},
    unit::Unit,
};

const CONVERSION_RANGE: f32 = 60.0;
const CONVERSION_RATE: f32 = 0.4;

pub struct ConversionPlugin;

impl Plugin for ConversionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                assign_converters,
                unassign_converters,
                pursue_prey,
                persuade_unit,
                convert_unfaithful_units,
            ),
        )
        .add_event::<AssignConvertPursuit>()
        .add_event::<Convert>();
    }
}

#[derive(Event)]
pub struct AssignConvertPursuit {
    pub predators: Vec<Entity>,
    pub prey: Entity,
}

#[derive(Component)]
pub struct ConvertPursuit {
    pub cooldown: f32,
    pub prey: Entity,
}

#[derive(Component)]
pub struct Faith {
    pub base: f32,
    pub current: f32,
}

#[derive(Event)]
pub struct Convert {
    pub victim: Entity,
    pub value: f32,
}

fn assign_converters(mut assignments: EventReader<AssignConvertPursuit>, mut commands: Commands) {
    for assignment in assignments.read() {
        for &predator in assignment.predators.iter() {
            //  clear away existing convert pursuit before assigning one
            commands
                .entity(predator)
                .remove::<ConvertPursuit>()
                .insert(ConvertPursuit {
                    cooldown: 0.0,
                    prey: assignment.prey,
                });
        }
    }
}

fn unassign_converters(
    predators: Query<(Entity, &ConvertPursuit, &Team), With<ConvertPursuit>>,
    victims: Query<&Team, With<Faith>>,
    mut commands: Commands,
) {
    for (entity, convert_pursuit, predator_team) in predators.iter() {
        if let Ok(prey_team) = victims.get(convert_pursuit.prey) {
            //	if they just converted, they need no more persuation
            if predator_team.0 == prey_team.0 {
                commands.entity(entity).remove::<ConvertPursuit>();
            }
        }
    }
}

fn pursue_prey(
    mut predators: Query<(&mut ConvertPursuit, &Transform, &Priest, &Team), With<ConvertPursuit>>,
    victims: Query<&Transform, With<Faith>>,
    time: Res<Time>,
    mut movement_writer: EventWriter<SetUnitPosition>,
    mut convert_events: EventWriter<Convert>,
) {
    for (mut convert_pursuit, predator_transform, priest, team) in predators.iter_mut() {
        convert_pursuit.cooldown -= time.delta_seconds();

        if convert_pursuit.cooldown < 0.0 {
            if let Ok(victim_transform) = victims.get(convert_pursuit.prey) {
                let dist = predator_transform
                    .translation
                    .distance(victim_transform.translation);

                if dist > CONVERSION_RANGE {
                    let convert_direction = (predator_transform.translation
                        - victim_transform.translation)
                        .normalize()
                        .xy();
                    movement_writer.send(SetUnitPosition {
                        position: victim_transform.translation.xy(),
                        direction: convert_direction,
                        formation: Formation::Ringed,
                        team: team.0.clone(),
                    });
                } else {
                    convert_events.send(Convert {
                        victim: convert_pursuit.prey,
                        value: priest.persuation,
                    });
                }

                convert_pursuit.cooldown = CONVERSION_RATE;
            }
        }
    }
}

fn persuade_unit(
    mut convert_events: EventReader<Convert>,
    mut victim_faith: Query<&mut Faith, With<Unit>>,
) {
    for convert in convert_events.read() {
        if let Ok(mut faith) = victim_faith.get_mut(convert.victim) {
            faith.current -= convert.value;
        }
    }
}

fn convert_unfaithful_units(mut query: Query<(&mut Faith, &mut Team), With<Faith>>) {
    for (mut faith, mut team) in query.iter_mut() {
        if faith.current < 0.0 {
            team.0 = match team.0.clone() {
                TeamType::Human => TeamType::CPU,
                TeamType::CPU => TeamType::Human,
            };

            faith.current = faith.base;
        }
    }
}
