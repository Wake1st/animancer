use bevy::prelude::*;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Energy { value: 500.0 });
    }
}

#[derive(Resource)]
pub struct Energy {
    pub value: f32,
}
