use bevy::prelude::*;

use crate::{
    combat::{assign_attackers, AssignAttackPursuit, AttackPursuit, Health},
    conversion::{assign_converters, AssignConvertPursuit, ConvertPursuit, Faith},
    priest::Priest,
    schedule::InGameSet,
    teams::Team,
    warrior::Warrior,
};

pub struct DetectionPlugin;

impl Plugin for DetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detect_attack_target.before(assign_attackers),
                detect_convert_target.before(assign_converters),
            )
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Component)]
pub struct Detector {
    pub range: f32,
}

fn detect_attack_target(
    detectors: Query<
        (Entity, &GlobalTransform, &Team, &Detector),
        (With<Warrior>, Without<AttackPursuit>),
    >,
    targets: Query<(Entity, &GlobalTransform, &Team), With<Health>>,
    mut event: EventWriter<AssignAttackPursuit>,
) {
    for (target_entity, target_transform, target_team) in targets.iter() {
        let mut pursuers: Vec<Entity> = Vec::new();

        for (detector_entity, detector_transform, detector_team, detector_detection) in
            detectors.iter()
        {
            //  detector should not "detect" itself or it's own team
            if detector_entity == target_entity || detector_team.0 == target_team.0 {
                continue;
            }

            let distance = detector_transform
                .translation()
                .distance(target_transform.translation());

            if distance < detector_detection.range {
                pursuers.push(detector_entity);
            }
        }

        if pursuers.len() > 0 {
            event.send(AssignAttackPursuit {
                predators: pursuers,
                prey: target_entity,
            });
        }
    }
}

fn detect_convert_target(
    detectors: Query<
        (Entity, &GlobalTransform, &Team, &Detector),
        (With<Priest>, Without<ConvertPursuit>),
    >,
    targets: Query<(Entity, &GlobalTransform, &Team), With<Faith>>,
    mut event: EventWriter<AssignConvertPursuit>,
) {
    for (target_entity, target_transform, target_team) in targets.iter() {
        let mut pursuers: Vec<Entity> = Vec::new();

        for (detector_entity, detector_transform, detector_team, detector_detection) in
            detectors.iter()
        {
            //  detector should not "detect" itself or it's own team
            if detector_entity == target_entity || detector_team.0 == target_team.0 {
                continue;
            }

            let distance = detector_transform
                .translation()
                .distance(target_transform.translation());

            if distance < detector_detection.range {
                pursuers.push(detector_entity);
            }
        }

        if pursuers.len() > 0 {
            event.send(AssignConvertPursuit {
                predators: pursuers,
                prey: target_entity,
            });
        }
    }
}
