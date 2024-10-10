use bevy::prelude::*;

use crate::{currency::Energy, schedule::InGameSet, structure::Structure, worker::Worker};

const GENERATOR_BASE_RATE: f32 = 1.0;
const WORKING_RANGE: f32 = 60.0;

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    assign_new_workers,
                    (set_assigned_workers, set_working_workers).chain(),
                ),
                (get_worker_effort, generate),
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<AssignGeneratorWorkers>();
    }
}

#[derive(Component)]
pub struct Generator {
    pub gen_type: GeneratorType,
    pub is_running: bool,
    pub base_rate: f32,
    pub added_rate: f32,
    pub assigned_workers: Vec<Entity>,
    pub working_workers: Vec<Entity>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            gen_type: GeneratorType::Energy,
            is_running: true,
            base_rate: GENERATOR_BASE_RATE,
            added_rate: 0.0,
            assigned_workers: Vec::new(),
            working_workers: Vec::new(),
        }
    }
}

pub enum GeneratorType {
    Energy,
}

#[derive(Event)]
pub struct AssignGeneratorWorkers {
    pub generator: Entity,
    pub workers: Vec<Entity>,
}

fn assign_new_workers(
    mut assignment_event: EventReader<AssignGeneratorWorkers>,
    mut generator_query: Query<&mut Generator>,
) {
    for assignment in assignment_event.read() {
        if let Ok(mut generator) = generator_query.get_mut(assignment.generator) {
            for entity in assignment.workers.iter() {
                generator.assigned_workers.push(*entity);
            }
        }
    }
}

fn set_assigned_workers(
    mut generator_query: Query<(&Transform, &mut Generator)>,
    workers: Query<&Transform, With<Worker>>,
) {
    for (generator_transform, mut generator) in generator_query.iter_mut() {
        let mut swapping: Vec<(usize, Entity)> = vec![];

        //	store all the working workers far enough to swap
        for (index, &entity) in generator.working_workers.iter().enumerate() {
            if let Ok(worker_transform) = workers.get(entity) {
                if generator_transform
                    .translation
                    .distance(worker_transform.translation)
                    > WORKING_RANGE
                {
                    swapping.push((index, entity));
                }
            }
        }

        //	swap the working workers to assigned workers
        for (index, entity) in swapping.iter() {
            generator.assigned_workers.push(*entity);
            generator.working_workers.remove(*index);
        }
    }
}

fn set_working_workers(
    mut generator_query: Query<(&Transform, &mut Generator)>,
    workers: Query<&Transform, With<Worker>>,
) {
    for (generator_transform, mut generator) in generator_query.iter_mut() {
        let mut swapping: Vec<(usize, Entity)> = vec![];

        //	store all the assigned workers close enough to swap
        for (index, &entity) in generator.assigned_workers.iter().enumerate() {
            if let Ok(worker_transform) = workers.get(entity) {
                if generator_transform
                    .translation
                    .distance(worker_transform.translation)
                    < WORKING_RANGE
                {
                    swapping.push((index, entity));
                }
            }
        }

        //	swap the assigned workers to working workers
        for (index, entity) in swapping.iter().rev() {
            generator.working_workers.push(*entity);
            generator.assigned_workers.remove(*index);
        }
    }
}

fn get_worker_effort(mut generator_query: Query<&mut Generator>, worker_query: Query<&Worker>) {
    let mut total_effort: f32 = 0.0;

    for mut generator in generator_query.iter_mut() {
        for &entity in generator.working_workers.iter() {
            if let Ok(worker) = worker_query.get(entity) {
                total_effort += worker.effort;
            }
        }

        generator.added_rate = total_effort;
    }
}

fn generate(
    time: Res<Time>,
    mut energy: ResMut<Energy>,
    query: Query<&Generator, With<Structure>>,
) {
    let delta_time = time.delta_seconds();

    for generator in query.iter() {
        if generator.is_running {
            match generator.gen_type {
                GeneratorType::Energy => {
                    energy.value += (generator.base_rate + generator.added_rate) * delta_time
                }
            }
        }
    }
}
