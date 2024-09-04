use bevy::{
    color::palettes::{
        css::DARK_GREEN,
        tailwind::{GRAY_200, GRAY_800, GREEN_200},
    },
    prelude::*,
};

use crate::{
    inputs::BuildSelection,
    schedule::InGameSet,
    structure::StructureType,
    worker::{DisplayWorkerUI, RemoveWorkerUI},
};

const MARGIN: Val = Val::Px(12.);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.35, 0.35);

const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
const WORKER_PRODUCER_ASSET_PATH: &str = "worker producer.png";

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        app.add_systems(Update, (remove_worker_ui, display_worker_ui).chain())
            .add_systems(Update, button_system.in_set(InGameSet::UIInput))
            .insert_resource(CurrentUI { focused: false });
    }
}

#[derive(Component)]
struct BuildButton {
    structure_type: StructureType,
}

#[derive(Component)]
pub struct WorkerUI {}

#[derive(Resource)]
pub struct CurrentUI {
    pub focused: bool,
}

fn display_worker_ui(
    mut display_worker_ui: EventReader<DisplayWorkerUI>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in display_worker_ui.read() {
        let simple_shrine_texture: Handle<Image> = asset_server.load(SIMPLE_SHRINE_ASSET_PATH);
        let worker_producer_texture: Handle<Image> = asset_server.load(WORKER_PRODUCER_ASSET_PATH);

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        // fill the entire window
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexStart,
                        row_gap: MARGIN,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                WorkerUI {},
            ))
            .with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            padding: UiRect::all(MARGIN),
                            ..Default::default()
                        },
                        background_color: Color::Srgba(DARK_GREEN).into(),
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: Color::Srgba(GRAY_800).into(),
                                background_color: NORMAL_BUTTON.into(),
                                image: UiImage {
                                    texture: simple_shrine_texture,
                                    ..default()
                                },
                                ..default()
                            },
                            BuildButton {
                                structure_type: StructureType::SimpleShrine,
                            },
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: Color::Srgba(GRAY_800).into(),
                                background_color: NORMAL_BUTTON.into(),
                                image: UiImage {
                                    texture: worker_producer_texture,
                                    ..default()
                                },
                                ..default()
                            },
                            BuildButton {
                                structure_type: StructureType::WorkerProducer,
                            },
                        ));
                    });
            });
    }
}

fn remove_worker_ui(
    mut remove_worker_ui: EventReader<RemoveWorkerUI>,
    mut worker_ui_query: Query<Entity, With<WorkerUI>>,
    mut commands: Commands,
) {
    for _ in remove_worker_ui.read() {
        info!("event");
        for entity in &mut worker_ui_query {
            info!("query: {:?}", entity); //  TODO: figure out why this is fucked
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &BuildButton),
        (Changed<Interaction>, With<BuildButton>),
    >,
    mut build_selection: ResMut<BuildSelection>,
    mut current_ui: ResMut<CurrentUI>,
) {
    current_ui.focused = false;
    for (interaction, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::Srgba(GREEN_200);
                build_selection.structure_type = button.structure_type.clone();
                build_selection.is_selected = true;
                current_ui.focused = true;
            }
            Interaction::Hovered => {
                border_color.0 = Color::Srgba(GRAY_200);
                current_ui.focused = true;
            }
            Interaction::None => {
                border_color.0 = Color::Srgba(GRAY_800);
                current_ui.focused = current_ui.focused || false;
            }
        }
    }
}
