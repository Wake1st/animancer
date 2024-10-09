use bevy::prelude::*;

use crate::{
    movement::{Formation, SetUnitPosition},
    unit::Unit,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (assign_attackers, attack_unit, destroy_unhealthy_units),
        )
        .add_event::<AssignAttackPursuit>()
        .add_event::<Attack>();
    }
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Event)]
pub struct Attack {
    pub victim: Entity,
    pub value: f32,
}

#[derive(Event)]
pub struct AssignAttackPursuit {
    pub predators: Vec<Entity>,
    pub prey: Entity,
}

fn assign_attackers(
    mut assignments: EventReader<AssignAttackPursuit>,
    victims: Query<&Transform, With<Health>>,
    mut movement_writer: EventWriter<SetUnitPosition>,
) {
    for assignment in assignments.read() {
        if let Ok(victim_transform) = victims.get(assignment.prey) {
            movement_writer.send(SetUnitPosition {
                position: victim_transform.translation.xy(),
                direction: Vec2::ONE,
                formation: Formation::Ringed,
            });
        }
    }
}

fn attack_unit(
    mut attack_events: EventReader<Attack>,
    mut victim_health: Query<&mut Health, With<Unit>>,
) {
    for attack in attack_events.read() {
        if let Ok(mut health) = victim_health.get_mut(attack.victim) {
            health.0 -= attack.value;
        }
    }
}

fn destroy_unhealthy_units(query: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.0 < 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
