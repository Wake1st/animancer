use std::cmp::Ordering;

use bevy::{
    color::palettes::tailwind::{GRAY_100, GREEN_300},
    input::mouse::MouseWheel,
    math::vec2,
    prelude::*,
};

use crate::{
    movement::{Formation, UnitMovement},
    selectable::BoxSelection,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_helper_text)
            .add_systems(
                Update,
                (
                    (handle_click, handle_mouse_wheel),
                    (draw_box_selection, draw_unit_aim, text_update_system),
                )
                    .chain(),
            )
            .insert_resource(BoxSelector {
                selecting: false,
                start: Default::default(),
                current: Default::default(),
                form: Formation::Line,
            })
            .insert_resource(UnitAim {
                aiming: false,
                start: Default::default(),
                current: Default::default(),
            });
    }
}

#[derive(Resource)]
struct BoxSelector {
    selecting: bool,
    start: Vec2,
    current: Vec2,
    form: Formation,
}

#[derive(Resource)]
struct UnitAim {
    aiming: bool,
    start: Vec2,
    current: Vec2,
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut box_selector: ResMut<BoxSelector>,
    mut unit_aim: ResMut<UnitAim>,
    mut box_selection_writer: EventWriter<BoxSelection>,
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
            box_selection_writer.send(BoxSelection {
                rect: Rect::from_corners(box_selector.start, box_selector.current),
            });

            box_selector.selecting = false;
            box_selector.start = Vec2::ZERO;
            box_selector.current = Vec2::ZERO;
        } else if mouse_button_input.pressed(MouseButton::Right) {
            if unit_aim.aiming == false {
                unit_aim.aiming = true;
                unit_aim.start = pos;
            } else {
                unit_aim.current = pos;
            }
        } else if mouse_button_input.just_released(MouseButton::Right) && unit_aim.aiming {
            movement_writer.send(UnitMovement {
                pos: unit_aim.start,
                dir: unit_aim.current - unit_aim.start,
                form: box_selector.form.clone(),
            });

            unit_aim.aiming = false;
            unit_aim.start = Vec2::ZERO;
            unit_aim.current = Vec2::ZERO;
        }
    }
}

fn handle_mouse_wheel(
    mut mouse_wheel_input: EventReader<MouseWheel>,
    mut box_selector: ResMut<BoxSelector>,
) {
    for wheel in mouse_wheel_input.read() {
        box_selector.form = match (wheel.y.total_cmp(&0.0), box_selector.form.clone()) {
            (Ordering::Less, Formation::Ringed) => Formation::Line,
            (Ordering::Less, Formation::Line) => Formation::Ringed,
            (Ordering::Greater, Formation::Ringed) => Formation::Line,
            (Ordering::Greater, Formation::Line) => Formation::Ringed,
            _ => box_selector.form.clone(),
        }
    }
}

fn draw_box_selection(box_selector: Res<BoxSelector>, mut gizmos: Gizmos) {
    if box_selector.selecting {
        gizmos.linestrip_2d(
            [
                vec2(box_selector.start.x, box_selector.start.y),
                vec2(box_selector.current.x, box_selector.start.y),
                vec2(box_selector.current.x, box_selector.current.y),
                vec2(box_selector.start.x, box_selector.current.y),
                vec2(box_selector.start.x, box_selector.start.y),
            ],
            GRAY_100,
        );
    }
}

fn draw_unit_aim(unit_aim: Res<UnitAim>, mut gizmos: Gizmos) {
    if unit_aim.aiming {
        gizmos.linestrip_2d([unit_aim.start, unit_aim.current], GREEN_300);
    }
}

#[derive(Component)]
struct FormationDebugText;

fn setup_debug_helper_text(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new("Formation::", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }),
        ]),
        FormationDebugText,
    ));
}

fn text_update_system(
    box_selector: Res<BoxSelector>,
    mut query: Query<&mut Text, With<FormationDebugText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = (match box_selector.form {
            Formation::Line => "Line",
            Formation::Ringed => "Ringed",
        })
        .into();
    }
}
