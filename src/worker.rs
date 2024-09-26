use bevy::prelude::*;

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisplayWorkerUI>()
            .add_event::<RemoveWorkerUI>();
    }
}

#[derive(Component)]
pub struct Worker {
    pub effort: f32,
}

#[derive(Event)]
pub struct DisplayWorkerUI {}

#[derive(Event)]
pub struct RemoveWorkerUI {}
