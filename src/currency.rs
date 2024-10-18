use bevy::prelude::*;

use crate::teams::TeamType;

const CURRENCY_START: f32 = 100.0;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Energy {
            human: CURRENCY_START,
            cpu: CURRENCY_START,
        });
    }
}

#[derive(Resource)]
pub struct Energy {
    pub human: f32,
    pub cpu: f32,
}

impl Energy {
    pub fn get(&self, team: &TeamType) -> f32 {
        match team {
            TeamType::Human => self.human,
            TeamType::CPU => self.cpu,
        }
    }

    pub fn add(&mut self, team: &TeamType, value: f32) {
        match team {
            TeamType::Human => {
                self.human += value;
            }
            TeamType::CPU => {
                self.cpu += value;
            }
        }
    }
}
