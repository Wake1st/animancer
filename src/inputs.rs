use bevy::{color::palettes::tailwind::GRAY_100, math::vec3, prelude::*};

use crate::{
    movement::UnitMovement,
    selectable::{BoxSelection, UnitSelection},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_click, draw_box_selection).chain())
            .insert_resource(BoxSelector {
                selecting: false,
                start: Vec2::ZERO,
                current: Vec2::ZERO,
            });
    }
}

#[derive(Resource)]
struct BoxSelector {
    selecting: bool,
    start: Vec2,
    current: Vec2,
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut box_selector: ResMut<BoxSelector>,
    mut box_selection_writer: EventWriter<BoxSelection>,
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
        if mouse_button_input.pressed(MouseButton::Left) {
            if box_selector.selecting == false {
                box_selector.selecting = true;
                box_selector.start = pos;
            } else {
                box_selector.current = pos;
            }
        } else if mouse_button_input.just_released(MouseButton::Left) && box_selector.selecting {
            let mut box_selection = BoxSelection {
                left: 0.,
                top: 0.,
                right: 0.,
                bottom: 0.,
            };

            if box_selector.start.x < box_selector.current.x {
                box_selection.left = box_selector.start.x;
                box_selection.right = box_selector.current.x;
            } else {
                box_selection.left = box_selector.current.x;
                box_selection.right = box_selector.start.x;
            }

            if box_selector.start.y < box_selector.current.y {
                box_selection.top = box_selector.current.y;
                box_selection.bottom = box_selector.start.y;
            } else {
                box_selection.top = box_selector.start.y;
                box_selection.bottom = box_selector.current.y;
            }

            box_selection_writer.send(box_selection);

            box_selector.selecting = false;
            box_selector.start = Vec2::ZERO;
            box_selector.current = Vec2::ZERO;
        } else if mouse_button_input.just_pressed(MouseButton::Left) {
            selection_writer.send(UnitSelection { pos });
        } else if mouse_button_input.just_pressed(MouseButton::Right) {
            movement_writer.send(UnitMovement { pos });
        }
    }
}

fn draw_box_selection(box_selector: Res<BoxSelector>, mut gizmos: Gizmos) {
    if box_selector.selecting {
        gizmos.linestrip(
            [
                vec3(box_selector.start.x, box_selector.start.y, 0.0),
                vec3(box_selector.current.x, box_selector.start.y, 0.0),
                vec3(box_selector.current.x, box_selector.current.y, 0.0),
                vec3(box_selector.start.x, box_selector.current.y, 0.0),
                vec3(box_selector.start.x, box_selector.start.y, 0.0),
            ],
            GRAY_100,
        );
    }
}
