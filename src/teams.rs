use std::slice::Iter;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Team(pub TeamType);

#[derive(PartialEq, Debug)]
pub enum TeamType {
    Human,
    CPU,
}

impl Default for TeamType {
    fn default() -> Self {
        TeamType::Human
    }
}

impl Clone for TeamType {
    fn clone(&self) -> Self {
        match self {
            Self::Human => Self::Human,
            Self::CPU => Self::CPU,
        }
    }
}

pub struct TeamBasedValues<T> {
    pub human: Vec<T>,
    pub cpu: Vec<T>,
}

impl<T> Default for TeamBasedValues<T> {
    fn default() -> Self {
        Self {
            human: Default::default(),
            cpu: Default::default(),
        }
    }
}

impl<T> TeamBasedValues<T> {
    pub fn len(&self, team: &TeamType) -> usize {
        match team {
            TeamType::Human => self.human.len(),
            TeamType::CPU => self.cpu.len(),
        }
    }

    pub fn clear(&mut self, team: &TeamType) {
        match team {
            TeamType::Human => self.human.clear(),
            TeamType::CPU => self.cpu.clear(),
        }
    }

    pub fn push(&mut self, team: &TeamType, value: T) {
        match team {
            TeamType::Human => self.human.push(value),
            TeamType::CPU => self.cpu.push(value),
        }
    }

    pub fn iter(&self, team: &TeamType) -> Iter<'_, T> {
        match team {
            TeamType::Human => self.human.iter(),
            TeamType::CPU => self.cpu.iter(),
        }
    }
}
