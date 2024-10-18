use bevy::prelude::*;

use crate::state::GameState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::UIInput,
                InGameSet::UserInput,
                InGameSet::AIInput,
                InGameSet::SelectionState,
                InGameSet::SpawnEntities,
                InGameSet::EntityUpdates,
                InGameSet::ConvertEntities,
                InGameSet::DespawnEntities,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UIInput,
    UserInput,
    AIInput,
    SelectionState,
    SpawnEntities,
    EntityUpdates,
    ConvertEntities,
    DespawnEntities,
}
