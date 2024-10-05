use bevy::prelude::*;

pub struct PriestPlugin;

impl Plugin for PriestPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component)]
pub struct Priest {
    pub effort: f32,
}
