use bevy::prelude::*;

const ATTACK_RANGE: f32 = 40.0;
const ATTACK_RATE: f32 = 0.4;

use crate::{
    movement::{Formation, SetUnitPosition},
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
                pursue_prey,
                attack_unit,
                destroy_unhealthy_units,
            ),
        )
        .add_event::<AssignAttackPursuit>()
        .add_event::<Attack>();
    }
}

#[derive(Event)]
pub struct AssignAttackPursuit {
    pub predators: Vec<Entity>,
    pub prey: Entity,
}

#[derive(Component)]
pub struct AttackPursuit {
    pub cooldown: f32,
    pub prey: Entity,
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Event)]
pub struct Attack {
    pub victim: Entity,
    pub value: f32,
}

fn assign_attackers(mut assignments: EventReader<AssignAttackPursuit>, mut commands: Commands) {
    for assignment in assignments.read() {
        for &predator in assignment.predators.iter() {
            //  clear away existing attack pursuit before assigning one
            commands
                .entity(predator)
                .remove::<AttackPursuit>()
                .insert(AttackPursuit {
                    cooldown: 0.0,
                    prey: assignment.prey,
                });
        }
    }
}

fn pursue_prey(
    mut predators: Query<
        (Entity, &mut AttackPursuit, &Transform, &Warrior, &Team),
        With<AttackPursuit>,
    >,
    victims: Query<&Transform, With<Health>>,
    time: Res<Time>,
    mut movement_writer: EventWriter<SetUnitPosition>,
    mut attack_events: EventWriter<Attack>,
    mut commands: Commands,
) {
    for (predetor_entity, mut attack_pursuit, predator_transform, warrior, team) in
        predators.iter_mut()
    {
        attack_pursuit.cooldown -= time.delta_seconds();

        if attack_pursuit.cooldown < 0.0 {
            if let Ok(victim_transform) = victims.get(attack_pursuit.prey) {
                let dist = predator_transform
                    .translation
                    .distance(victim_transform.translation);

                if dist > ATTACK_RANGE {
                    let attack_direction = (predator_transform.translation
                        - victim_transform.translation)
                        .normalize()
                        .xy();
                    movement_writer.send(SetUnitPosition {
                        position: victim_transform.translation.xy(),
                        direction: attack_direction,
                        formation: Formation::Ringed,
                        team: team.0.clone(),
                    });
                } else {
                    attack_events.send(Attack {
                        victim: attack_pursuit.prey,
                        value: warrior.strength,
                    });
                }

                attack_pursuit.cooldown = ATTACK_RATE;
            } else {
                //  If we cannot find prey, then it is most likely dead
                commands.entity(predetor_entity).remove::<AttackPursuit>();
            }
        }
    }
}

fn break_pursuit() {}

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
