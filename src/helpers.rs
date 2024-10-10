use bevy::{
    color::palettes::{
        // self,
        tailwind::{GRAY_100, GREEN_300},
    },
    math::vec2,
    prelude::*,
    // sprite::MaterialMesh2dBundle,
    // window::WindowResized,
};
// use vleue_navigator::{prelude::NavMeshStatus, NavMesh};

use crate::{
    camera::CameraDirection,
    construction::{ConstructionSilhouette, Intersects},
    currency::Energy,
    inputs::{BoxSelector, BuildSelection, ProducerSelection, UnitAim},
    movement::Formation,
    // nav_agent::Path,
    selectable::{SelectionState, SelectionType},
    structure::StructureType,
    ui::{CurrentUI, UIType},
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
                    debug_text,
                    // display_mesh,
                    // display_navigator_path,
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
            TextSection::new("\nProducer - ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  6
            TextSection::new("\nFaith = ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  8
            TextSection::new("\nUI Focused = ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  10
            TextSection::new("\nUI Type::", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  12
            TextSection::new("\nSelectionState::", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  14
            TextSection::new("\nIntersection = ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  16
            TextSection::new("\nCamera Zoom = ", TextStyle { ..default() }),
            TextSection::new("_", TextStyle { ..default() }), //  18
        ]),
        DebugText,
    ));
}

fn debug_text(
    box_selector: Res<BoxSelector>,
    build_selection: Res<BuildSelection>,
    producer_selection: Res<ProducerSelection>,
    energy: Res<Energy>,
    current_ui: Res<CurrentUI>,
    selection_state: Res<SelectionState>,
    construction_silhouettes: Query<&Intersects, With<ConstructionSilhouette>>,
    query_camera: Query<&CameraDirection, With<Camera2d>>,
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
            StructureType::SimpleShrine => "SimpleShrine - ",
            StructureType::Producer => "WorkerProducer - ",
        })
        .into();

        text.sections[4].value = (match build_selection.is_selected {
            true => "is selected",
            false => "is not selected",
        })
        .into();

        text.sections[6].value = (match producer_selection.is_selected {
            true => "is selected",
            false => "is not selected",
        })
        .into();

        text.sections[8].value = energy.value.to_string();

        text.sections[10].value = current_ui.focused.to_string();

        text.sections[12].value = (match current_ui.ui_type {
            UIType::None => "None",
            UIType::Worker => "Worker",
            UIType::Producer => "Producer",
            UIType::Generator => "Generator",
        })
        .to_string();

        text.sections[14].value = (match selection_state.0 {
            SelectionType::None => "None",
            SelectionType::Unit => "Unit",
            SelectionType::Worker => "Worker",
            SelectionType::Priest => "Priest",
            SelectionType::Warrior => "Warrior",
            SelectionType::Construction => "Construction",
            SelectionType::Generator => "Generator",
            SelectionType::Producer => "Producer",
        })
        .to_string();

        for intersects in construction_silhouettes.iter() {
            text.sections[16].value = (match intersects.0 {
                true => "true",
                false => "false",
            })
            .into();
        }

        let direction = query_camera.single();
        text.sections[18].value = direction.height.to_string();
    }
}

// fn display_mesh(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     navmeshes: Res<Assets<NavMesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut current_mesh_entity: Local<Option<Entity>>,
//     window_resized: EventReader<WindowResized>,
//     navmesh: Query<(&Handle<NavMesh>, Ref<NavMeshStatus>)>,
// ) {
//     let (navmesh_handle, status) = navmesh.single();
//     if (!status.is_changed() || *status != NavMeshStatus::Built) && window_resized.is_empty() {
//         return;
//     }

//     let Some(navmesh) = navmeshes.get(navmesh_handle) else {
//         return;
//     };
//     if let Some(entity) = *current_mesh_entity {
//         commands.entity(entity).despawn_recursive();
//     }

//     *current_mesh_entity = Some(
//         commands
//             .spawn(MaterialMesh2dBundle {
//                 mesh: meshes.add(navmesh.to_mesh()).into(),
//                 material: materials.add(ColorMaterial::from(Color::Srgba(
//                     palettes::tailwind::BLUE_800,
//                 ))),
//                 ..default()
//             })
//             .with_children(|main_mesh| {
//                 main_mesh.spawn(MaterialMesh2dBundle {
//                     mesh: meshes.add(navmesh.to_wireframe_mesh()).into(),
//                     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 11.1)),
//                     material: materials.add(ColorMaterial::from(Color::Srgba(
//                         palettes::tailwind::TEAL_300,
//                     ))),
//                     ..default()
//                 });
//             })
//             .id(),
//     );
// }

// fn display_navigator_path(navigator: Query<(&Transform, &Path)>, mut gizmos: Gizmos) {
//     let Ok((transform, path)) = navigator.get_single() else {
//         return;
//     };
//     let mut to_display = path.next.clone();
//     to_display.push(path.current);
//     to_display.push(transform.translation.xy());
//     to_display.reverse();
//     if !to_display.is_empty() {
//         gizmos.linestrip_2d(to_display, palettes::css::YELLOW);
//     }
// }
