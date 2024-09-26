use std::cmp::Ordering;

use bevy::{input::mouse::MouseWheel, math::vec3, prelude::*};

use crate::{
    construction::PlaceConstructionSite,
    currency::Faith,
    movement::{Formation, UnitMovement},
    producer::{PostSpawnMarker, Producer},
    schedule::InGameSet,
    selectable::{
        BoxSelection, SelectedStructures, SelectionState, SelectionStateChanged, SelectionType,
    },
    structure::StructureType,
    ui::CurrentUI,
    unit::UnitAction,
};

const WINDOW_HEIGHT: f32 = 1080.;
const UI_BASE_HEIGHT: f32 = 88.;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_mouse_position, store_mouse_position)
                .chain()
                .in_set(InGameSet::UIInput),
        )
        .add_systems(
            Update,
            ((handle_click, handle_mouse_wheel), set_selection_state)
                .chain()
                .in_set(InGameSet::UserInput),
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
            cost: 0.,
        })
        .insert_resource(ProducerSelection { is_selected: false })
        .insert_resource(MousePosition(Vec2::ZERO));
    }
}

#[derive(Resource)]
pub struct MousePosition(pub Vec2);

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
    pub cost: f32,
}

#[derive(Resource)]
pub struct ProducerSelection {
    pub is_selected: bool,
}

/// check if cursor is hovered over ui
fn check_mouse_position(windows: Query<&Window>, mut current_ui: ResMut<CurrentUI>) {
    if let Some(pos) = windows.single().cursor_position() {
        current_ui.focused = pos.y > (WINDOW_HEIGHT - UI_BASE_HEIGHT);
    }
}

fn store_mouse_position(
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.0 = pos;
    }
}

fn handle_click(
    current_ui: Res<CurrentUI>,
    mouse_position: Res<MousePosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut box_selector: ResMut<BoxSelector>,
    mut unit_aim: ResMut<UnitAim>,
    box_selection_writer: EventWriter<BoxSelection>,
    mut movement_writer: EventWriter<UnitMovement>,
    build_selection: ResMut<BuildSelection>,
    mut place_construction_site: EventWriter<PlaceConstructionSite>,
    selected_structures: Res<SelectedStructures>,
    mut producers: Query<(&mut Producer, &Children)>,
    mut post_spawn_markers: Query<&mut PostSpawnMarker>,
    mut faith: ResMut<Faith>,
    selection_state: Res<SelectionState>,
    mut unit_action: EventWriter<UnitAction>,
) {
    //  ensure cursor is not hovered over ui
    if current_ui.focused {
        return;
    }

    let pos = mouse_position.0;

    match selection_state.0 {
        SelectionType::None => {
            click_selection(
                pos,
                &mouse_button_input,
                &mut box_selector,
                box_selection_writer,
            );
        }
        SelectionType::Unit => {
            click_selection(
                pos,
                &mouse_button_input,
                &mut box_selector,
                box_selection_writer,
            );

            if mouse_button_input.just_pressed(MouseButton::Right) {
                unit_action.send(UnitAction { position: pos });
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
        SelectionType::Construction => {
            if mouse_button_input.just_released(MouseButton::Left)
                && faith.value > build_selection.cost
            {
                faith.value -= build_selection.cost;

                place_construction_site.send(PlaceConstructionSite {
                    structure_type: build_selection.structure_type.clone(),
                    position: pos,
                    effort: build_selection.cost,
                });

                //  ensure units move to build
                movement_writer.send(UnitMovement {
                    position: pos,
                    direction: Vec2::ZERO,
                    formation: Formation::Ringed,
                });
            }
        }
        SelectionType::Building => {
            if mouse_button_input.just_pressed(MouseButton::Right) {
                for &entity in selected_structures.entities.iter() {
                    if let Ok((mut producer, children)) = producers.get_mut(entity) {
                        producer.post_spawn_location = vec3(pos.x, pos.y, 0.1);

                        for &child in children.iter() {
                            if let Ok(mut marker) = post_spawn_markers.get_mut(child) {
                                marker.not_set = false;
                            }
                        }
                    }
                }
            } else if mouse_button_input.pressed(MouseButton::Left)
                || mouse_button_input.just_released(MouseButton::Left)
            {
                click_selection(
                    pos,
                    &mouse_button_input,
                    &mut box_selector,
                    box_selection_writer,
                );
            }
        }
    }
}

fn set_selection_state(
    current_ui: Res<CurrentUI>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    selection_state: Res<SelectionState>,
    mut selection_state_changed: EventWriter<SelectionStateChanged>,
    faith: Res<Faith>,
    build_selection: ResMut<BuildSelection>,
) {
    //  ensure cursor is not hovered over ui
    if current_ui.focused {
        return;
    }

    match selection_state.0 {
        SelectionType::Construction => {
            let deselect_construction = mouse_button_input.just_released(MouseButton::Right);
            let place_structure = mouse_button_input.just_released(MouseButton::Left)
                && faith.value > build_selection.cost
                && !keys.pressed(KeyCode::ShiftLeft);

            if deselect_construction || place_structure {
                selection_state_changed.send(SelectionStateChanged {
                    new_type: SelectionType::Unit,
                });
            }
        }
        SelectionType::Building => {
            if mouse_button_input.pressed(MouseButton::Left)
                || mouse_button_input.just_released(MouseButton::Left)
            {
                selection_state_changed.send(SelectionStateChanged {
                    new_type: SelectionType::None,
                });
            }
        }
        _ => (),
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

fn click_selection(
    pos: Vec2,
    mouse_button_input: &Res<ButtonInput<MouseButton>>,
    box_selector: &mut BoxSelector,
    mut box_selection_writer: EventWriter<BoxSelection>,
) {
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
    }
}
