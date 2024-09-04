use bevy::prelude::*;

use crate::selectable::SelectedUnits;

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_build_ui)
            .add_event::<DisplayWorkerUI>()
            .add_event::<RemoveWorkerUI>();
    }
}

#[derive(Component)]
pub struct Worker {
    pub output: f32,
}

#[derive(Event)]
pub struct DisplayWorkerUI {}

#[derive(Event)]
pub struct RemoveWorkerUI {}

fn display_build_ui(
    selected_units: Res<SelectedUnits>,
    mut event_writer: EventWriter<DisplayWorkerUI>,
) {
    if selected_units.entities.len() > 0 {
        event_writer.send(DisplayWorkerUI {});
    }
}
