use bevy::prelude::*;

#[derive(Component)]
pub struct Team(pub TeamType);

#[derive(PartialEq)]
pub enum TeamType {
    Human,
    CPU,
}
