use bevy::prelude::*;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Faith { value: 500.0 });
    }
}

#[derive(Resource)]
pub struct Faith {
    pub value: f32,
}
