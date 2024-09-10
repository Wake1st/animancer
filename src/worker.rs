use bevy::prelude::*;

use crate::{
    selectable::SelectedUnits,
    ui::{CurrentUI, UIType},
};

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_worker_ui)
            .add_event::<DisplayWorkerUI>()
            .add_event::<RemoveWorkerUI>();
    }
}

#[derive(Component)]
pub struct Worker {}

#[derive(Event)]
pub struct DisplayWorkerUI {}

#[derive(Event)]
pub struct RemoveWorkerUI {}

fn display_worker_ui(
    selected_units: Res<SelectedUnits>,
    mut current_ui: ResMut<CurrentUI>,
    mut event_writer: EventWriter<DisplayWorkerUI>,
) {
    if selected_units.entities.len() > 0 {
        match current_ui.ui_type {
            UIType::None => {
                event_writer.send(DisplayWorkerUI {});
                current_ui.ui_type = UIType::Worker;
            }
            UIType::Worker => (),
            UIType::Producer => (),
        }
    }
}