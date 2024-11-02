use bevy::prelude::*;

const ATTACK_RANGE: f32 = 40.0;
const ATTACK_RATE: f32 = 0.4;

use crate::{
    movement::{Formation, SetUnitPosition},
    schedule::InGameSet,
    teams::Team,
    warrior::Warrior,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                assign_attackers,
                break_attack_pursuit,
                pursue_prey,
                attack_unit,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_systems(
            Update,
            destroy_unhealthy_units.in_set(InGameSet::DespawnEntities),
        )
        .add_event::<AssignAttackPursuit>()
        .add_event::<BreakAttackPursuit>()
        .add_event::<Attack>();
    }
}

#[derive(Component)]
pub struct CombatEngagementState(pub CombatEngagement);

pub enum CombatEngagement {
    None,
    Pursuing { target: Entity },
    Attacking { target: Entity, cooldown: f32 },
}

#[derive(Event)]
pub struct AssignAttackPursuit {
    pub predators: Vec<Entity>,
    pub prey: Entity,
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Event)]
pub struct Attack {
    pub victim: Entity,
    pub value: f32,
}

#[derive(Event)]
pub struct BreakAttackPursuit {
    pub entities: Vec<Entity>,
}

pub fn assign_attackers(
    mut assignments: EventReader<AssignAttackPursuit>,
    mut combatants: Query<&mut CombatEngagementState, With<Warrior>>,
    mut commands: Commands,
) {
    for assignment in assignments.read() {
        //  check to ensure prey exists
        if let None = commands.get_entity(assignment.prey) {
            continue;
        }

        for &predator in assignment.predators.iter() {
            //  check to ensure predator exists
            if let Ok(mut engagement_state) = combatants.get_entity(predator) {
                //  set new engagement
                engagement_state.0 = CombatEngagement::Attacking {
                    target: assignment.prey,
                    cooldown: ATTACK_RATE,
                };
            }
        }
    }
}

fn break_attack_pursuit(
    mut event: EventReader<BreakAttackPursuit>,
    mut combatants: Query<&mut CombatEngagementState, With<Warrior>>,
) {
    for break_attack in event.read() {
        for &entity in break_attack.entities.iter() {
            if let Ok(mut engagement_state) = combatants.get_entity(entity) {
                engagement_state.0 = CombatEngagement::None;
            }
        }
    }
}

fn pursue_prey(
    mut predators: Query<(
        Entity,
        &mut CombatEngagementState,
        &Transform,
        &Warrior,
        &Team,
    )>,
    targets: Query<&Transform, With<Health>>,
    mut movement_writer: EventWriter<SetUnitPosition>,
) {
    for (predetor_entity, mut engagement_state, predator_transform, warrior, team) in
        predators.iter_mut()
    {
        let mut new_engagement_state = engagement_state.0.clone();

        match engagement_state.0 {
            CombatEngagement::Pursuing { target } => {
                if let Ok(target_transform) = targets.get(target) {
                    let dist = predator_transform
                        .translation
                        .distance(target_transform.translation);

                    if dist > ATTACK_RANGE {
                        let attack_direction = (predator_transform.translation
                            - target_transform.translation)
                            .normalize()
                            .xy();

                        movement_writer.send(SetUnitPosition {
                            position: target_transform.translation.xy(),
                            direction: attack_direction,
                            formation: Formation::Ringed,
                            team: team.0.clone(),
                        });
                    } else {
                        new_engagement_state = CombatEngagement::Attacking {
                            target,
                            cooldown: ATTACK_RATE,
                        };
                    }
                } else {
                    //  If we cannot find prey, then it is most likely dead
                    new_engagement_state = CombatEngagement::None;
                }
            }
            _ => (),
        }

        if new_engagement_state != engagement_state.0 {
            engagement_state.0 = new_engagement_state.clone();
        }
    }
}

fn attacking(
    time: Res<Time>,
    mut combatants: Query<(&mut CombatEngagementState, &Warrior)>,
    mut attack_events: EventWriter<Attack>,
) {
    let delta_time = time.delta_seconds();

    for (mut engagement_state, warrior) in combatants.iter_mut() {
        match engagement_state.0 {
            CombatEngagement::Attacking {
                target,
                mut cooldown,
            } => {
                cooldown -= delta_time;

                if cooldown < 0.0 {
                    cooldown = ATTACK_RATE;

                    attack_events.send(Attack {
                        victim: target,
                        value: warrior.strength,
                    });
                }
            }
            _ => (),
        }
    }
}

fn attack_unit(mut attack_events: EventReader<Attack>, mut victim_health: Query<&mut Health>) {
    for attack in attack_events.read() {
        if let Ok(mut health) = victim_health.get_mut(attack.victim) {
            health.0 -= attack.value;
        }
    }
}

fn destroy_unhealthy_units(query: Query<(Entity, &Health), With<Health>>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.0 < 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
