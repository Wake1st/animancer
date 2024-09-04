use bevy::{math::vec3, prelude::*};

use crate::{faith::Faith, structure::Structure};

const SPAWN_OFFSET: Vec3 = vec3(0.0, -60.0, -0.1);

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate)
            .add_event::<GenerateWorker>();
    }
}

#[derive(Component)]
pub struct Generator {
    pub gen_type: GeneratorType,
    pub is_running: bool,
    pub queue: i32,
    pub value: f32,
    pub completion: f32,
    pub rate: f32,
}

pub enum GeneratorType {
    Faith,
    Worker,
}

#[derive(Event)]
pub struct GenerateWorker {
    pub position: Vec3,
}

fn generate(
    time: Res<Time>,
    mut faith: ResMut<Faith>,
    mut query: Query<(&mut Generator, &GlobalTransform), With<Structure>>,
    mut generation_writer: EventWriter<GenerateWorker>,
) {
    let delta_time = time.delta_seconds();

    for (mut generator, transform) in query.iter_mut() {
        if generator.is_running {
            match generator.gen_type {
                GeneratorType::Faith => faith.value += generator.rate * delta_time,
                GeneratorType::Worker => {
                    if generator.queue > 0 {
                        generator.value += generator.rate * delta_time;

                        if generator.value >= generator.completion {
                            //	leave the remainder, so as to avoid value loss over time
                            generator.value = generator.value % generator.completion;

                            generation_writer.send(GenerateWorker {
                                position: transform.translation() + SPAWN_OFFSET,
                            });
                            info!(
                                "worker generated at: {:?}",
                                transform.translation() + SPAWN_OFFSET
                            );
                        }
                    }
                }
            }
        }
    }
}
