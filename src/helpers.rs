use bevy::{
    color::palettes::tailwind::{GRAY_100, GREEN_300},
    math::{vec2, vec3},
    prelude::*,
};

use crate::{
    inputs::{BoxSelector, BuildSelection, UnitAim},
    movement::Formation,
    structure::StructureType,
};

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_helper_text)
            .add_systems(
                Update,
                (
                    draw_box_selection,
                    draw_unit_aim,
                    formation_text,
                    build_selection_text,
                ),
            );
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

#[derive(Component)]
struct GeneratorDebugText;

fn setup_debug_helper_text(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new("Formation::", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }),
        ]),
        FormationDebugText,
    ));

    let mut bundle = TextBundle::from_sections([
        TextSection::new("Structure::", TextStyle { ..default() }),
        TextSection::new("_", TextStyle { ..default() }),
        TextSection::new("_", TextStyle { ..default() }),
    ]);
    bundle.transform = Transform::from_translation(vec3(0.0, -80.0, 0.0));

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        bundle,
        GeneratorDebugText,
    ));
}

fn formation_text(
    box_selector: Res<BoxSelector>,
    mut query: Query<&mut Text, With<FormationDebugText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = (match box_selector.formation {
            Formation::Line => "Line",
            Formation::Ringed => "Ringed",
            Formation::Box => "Box",
        })
        .into();
    }
}

fn build_selection_text(
    build_selection: Res<BuildSelection>,
    mut query: Query<&mut Text, With<GeneratorDebugText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = (match build_selection.structure_type {
            StructureType::SimpleShrine => "SimpleShrine",
            StructureType::WorkerProducer => "WorkerProducer",
        })
        .into();

        text.sections[2].value = (match build_selection.is_selected {
            true => " -'is selected'",
            false => " -'is not selected'",
        })
        .into();
    }
}
