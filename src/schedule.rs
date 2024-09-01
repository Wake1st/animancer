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
                InGameSet::EntityUpdates,
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
    EntityUpdates,
}
