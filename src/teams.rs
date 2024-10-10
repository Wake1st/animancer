use bevy::prelude::*;

#[derive(Component)]
pub struct Team(pub TeamType);

pub enum TeamType {
    Human,
    CPU,
}
