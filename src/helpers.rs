use bevy::{
    color::palettes::tailwind::{GRAY_100, GREEN_300},
    math::vec2,
    prelude::*,
};

use crate::{
    faith::Faith,
    inputs::{BoxSelector, BuildSelection, UnitAim},
    movement::Formation,
    structure::StructureType,
    ui::CurrentUI,
};

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_helper_text)
            .add_systems(Update, (draw_box_selection, draw_unit_aim, debug_text));
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
struct DebugText;

fn setup_debug_helper_text(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new("Formation::", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  1
            TextSection::new("\nStructure::", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  3
            TextSection::new("_", TextStyle { ..default() }), //  4
            TextSection::new("\nFaith = ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  6
            TextSection::new("\nUI Focused = ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  8
        ]),
        DebugText,
    ));
}

fn debug_text(
    box_selector: Res<BoxSelector>,
    build_selection: Res<BuildSelection>,
    faith: Res<Faith>,
    current_ui: Res<CurrentUI>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = (match box_selector.formation {
            Formation::Line => "Line",
            Formation::Ringed => "Ringed",
            Formation::Box => "Box",
        })
        .into();

        text.sections[3].value = (match build_selection.structure_type {
            StructureType::SimpleShrine => "SimpleShrine",
            StructureType::WorkerProducer => "WorkerProducer",
        })
        .into();

        text.sections[4].value = (match build_selection.is_selected {
            true => " -'is selected'",
            false => " -'is not selected'",
        })
        .into();

        text.sections[6].value = faith.value.to_string();

        text.sections[8].value = current_ui.focused.to_string();
    }
}
