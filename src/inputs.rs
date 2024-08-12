use bevy::prelude::*;

use crate::{movement::UnitMovement, selectable::UnitSelection};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_click);
    }
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut selection_writer: EventWriter<UnitSelection>,
    mut movement_writer: EventWriter<UnitMovement>,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            selection_writer.send(UnitSelection { pos });
        } else if mouse_button_input.just_pressed(MouseButton::Right) {
            movement_writer.send(UnitMovement { pos });
        }
    }
}
