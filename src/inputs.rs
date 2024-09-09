use std::cmp::Ordering;

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    movement::{Formation, UnitMovement},
    schedule::InGameSet,
    selectable::BoxSelection,
    structure::{PlaceStructure, StructureType},
    ui::CurrentUI,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_click, handle_mouse_wheel).in_set(InGameSet::UserInput),
        )
        .insert_resource(BoxSelector {
            selecting: false,
            start: Default::default(),
            current: Default::default(),
            formation: Formation::Line,
        })
        .insert_resource(UnitAim {
            aiming: false,
            start: Default::default(),
            current: Default::default(),
        })
        .insert_resource(BuildSelection {
            is_selected: false,
            structure_type: StructureType::SimpleShrine,
        })
        .insert_resource(ProducerSelection { is_selected: false });
    }
}

#[derive(Resource)]
pub struct BoxSelector {
    pub selecting: bool,
    pub start: Vec2,
    pub current: Vec2,
    pub formation: Formation,
}

#[derive(Resource)]
pub struct UnitAim {
    pub aiming: bool,
    pub start: Vec2,
    pub current: Vec2,
}

#[derive(Resource)]
pub struct BuildSelection {
    pub is_selected: bool,
    pub structure_type: StructureType,
}

#[derive(Resource)]
pub struct ProducerSelection {
    pub is_selected: bool,
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    current_ui: Res<CurrentUI>,
    mut box_selector: ResMut<BoxSelector>,
    mut unit_aim: ResMut<UnitAim>,
    mut box_selection_writer: EventWriter<BoxSelection>,
    mut movement_writer: EventWriter<UnitMovement>,
    mut build_selection: ResMut<BuildSelection>,
    mut place_structure: EventWriter<PlaceStructure>,
    mut producer_selection: ResMut<ProducerSelection>,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        //  ensure cursor is not hovered over ui
        if current_ui.focused {
            return;
        }

        if build_selection.is_selected {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                place_structure.send(PlaceStructure {
                    structure_type: build_selection.structure_type.clone(),
                    position: pos,
                });
            } else if mouse_button_input.just_pressed(MouseButton::Right) {
                build_selection.is_selected = false;
            }
        } else if producer_selection.is_selected {
            if mouse_button_input.just_pressed(MouseButton::Right) {
                producer_selection.is_selected = false;
            }
        } else {
            if mouse_button_input.pressed(MouseButton::Left) {
                if box_selector.selecting == false {
                    box_selector.selecting = true;
                    box_selector.start = pos;
                } else {
                    box_selector.current = pos;
                }
            } else if mouse_button_input.just_released(MouseButton::Left) && box_selector.selecting
            {
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
                    position: unit_aim.start,
                    direction: unit_aim.current - unit_aim.start,
                    formation: box_selector.formation.clone(),
                });

                unit_aim.aiming = false;
                unit_aim.start = Vec2::ZERO;
                unit_aim.current = Vec2::ZERO;
            }
        }
    }
}

fn handle_mouse_wheel(
    mut mouse_wheel_input: EventReader<MouseWheel>,
    mut box_selector: ResMut<BoxSelector>,
) {
    for wheel in mouse_wheel_input.read() {
        box_selector.formation = match (wheel.y.total_cmp(&0.0), box_selector.formation.clone()) {
            (Ordering::Less, Formation::Ringed) => Formation::Line,
            (Ordering::Less, Formation::Line) => Formation::Box,
            (Ordering::Less, Formation::Box) => Formation::Ringed,
            (Ordering::Greater, Formation::Ringed) => Formation::Box,
            (Ordering::Greater, Formation::Line) => Formation::Ringed,
            (Ordering::Greater, Formation::Box) => Formation::Line,
            _ => box_selector.formation.clone(),
        }
    }
}
