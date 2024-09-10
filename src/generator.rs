use bevy::prelude::*;

use crate::{faith::Faith, structure::Structure};

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate);
    }
}

#[derive(Component)]
pub struct Generator {
    pub gen_type: GeneratorType,
    pub is_running: bool,
    pub rate: f32,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            gen_type: GeneratorType::Faith,
            is_running: true,
            rate: 1.0,
        }
    }
}

pub enum GeneratorType {
    Faith,
}

fn generate(time: Res<Time>, mut faith: ResMut<Faith>, query: Query<&Generator, With<Structure>>) {
    let delta_time = time.delta_seconds();

    for generator in query.iter() {
        if generator.is_running {
            match generator.gen_type {
                GeneratorType::Faith => faith.value += generator.rate * delta_time,
            }
        }
    }
}
