use bevy::prelude::*;

#[derive(Component)]
pub struct Warrior {
    pub strength: f32,
    pub attacking: bool,
}
