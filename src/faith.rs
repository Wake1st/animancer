use bevy::prelude::*;

pub struct FaithPlugin;

impl Plugin for FaithPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Faith { value: 100.0 });
    }
}

#[derive(Resource)]
pub struct Faith {
    pub value: f32,
}
