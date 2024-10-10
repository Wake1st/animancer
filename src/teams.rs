use bevy::prelude::*;

#[derive(Component)]
pub struct Team(pub TeamType);

#[derive(PartialEq)]
pub enum TeamType {
    Human,
    CPU,
}

impl Clone for TeamType {
    fn clone(&self) -> Self {
        match self {
            Self::Human => Self::Human,
            Self::CPU => Self::CPU,
        }
    }
}
