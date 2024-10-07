use bevy::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component)]
pub struct Warrior {
    pub strength: f32,
}
