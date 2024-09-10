use bevy::{
    color::palettes::{
        css::{DARK_GREEN, DARK_SLATE_BLUE},
        tailwind::{GRAY_200, GRAY_800, GREEN_200},
    },
    prelude::*,
};

use crate::{
    faith::Faith,
    inputs::BuildSelection,
    producer::{DisplayProducerUI, Producer, RemoveProducerUI},
    schedule::InGameSet,
    selectable::SelectedStructures,
    structure::StructureType,
    worker::{DisplayWorkerUI, RemoveWorkerUI},
};

const UI_BASE_HEIGHT: f32 = 88.;

const MARGIN: Val = Val::Px(12.);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.35, 0.35);

const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
const WORKER_PRODUCER_ASSET_PATH: &str = "worker producer.png";
const WORKER_ASSET_PATH: &str = "footman.png";

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        app.add_systems(Startup, (setup_ui_base, setup_worker_ui, setup_producer_ui))
            .add_systems(Update, (remove_worker_ui, display_worker_ui).chain())
            .add_systems(Update, (remove_producer_ui, display_producer_ui).chain())
            .add_systems(
                Update,
                (
                    build_button_interactions,
                    (producer_button_interactions, producer_queue_display).chain(),
                )
                    .in_set(InGameSet::UIInput),
            )
            .insert_resource(CurrentUI {
                focused: false,
                ui_type: UIType::None,
            });
    }
}

#[derive(Component)]
struct BuildButton {
    structure_type: StructureType,
}

#[derive(Component)]
struct ProducerButton {}

#[derive(Component)]
struct QueueText {}

#[derive(Component)]
pub struct WorkerUI {}

#[derive(Component)]
pub struct ProducerUI {}

pub enum UIType {
    None,
    Worker,
    Producer,
}

#[derive(Resource)]
pub struct CurrentUI {
    pub focused: bool,
    pub ui_type: UIType,
}

fn setup_ui_base(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
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
        },))
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(MARGIN),
                    height: Val::Px(UI_BASE_HEIGHT),
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                background_color: Color::Srgba(DARK_SLATE_BLUE).into(),
                ..Default::default()
            });
        });
}

fn setup_worker_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    display: Display::None,
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

fn setup_producer_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let worker_texture: Handle<Image> = asset_server.load(WORKER_ASSET_PATH);

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
                    display: Display::None,
                    ..Default::default()
                },
                ..Default::default()
            },
            ProducerUI {},
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
                    builder
                        .spawn((
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
                                    texture: worker_texture,
                                    ..default()
                                },
                                ..default()
                            },
                            ProducerButton {},
                        ))
                        .with_children(|builder| {
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        padding: UiRect::all(Val::Px(2.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexEnd,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    builder.spawn((
                                        TextBundle {
                                            text: Text::from_section("", TextStyle { ..default() }),
                                            ..default()
                                        },
                                        QueueText {},
                                    ));
                                });
                        });
                });
        });
}

fn display_worker_ui(
    mut display_worker_ui: EventReader<DisplayWorkerUI>,
    mut worker_ui_query: Query<&mut Style, With<WorkerUI>>,
) {
    for _ in display_worker_ui.read() {
        for mut style in &mut worker_ui_query {
            style.display = Display::Flex;
        }
    }
}

fn remove_worker_ui(
    mut remove_worker_ui: EventReader<RemoveWorkerUI>,
    mut worker_ui_query: Query<&mut Style, With<WorkerUI>>,
) {
    for _ in remove_worker_ui.read() {
        for mut style in &mut worker_ui_query {
            style.display = Display::None;
        }
    }
}

fn display_producer_ui(
    mut display_producer_ui: EventReader<DisplayProducerUI>,
    mut producer_ui_query: Query<&mut Style, With<ProducerUI>>,
) {
    for _ in display_producer_ui.read() {
        for mut style in &mut producer_ui_query {
            style.display = Display::Flex;
        }
    }
}

fn remove_producer_ui(
    mut remove_producer_ui: EventReader<RemoveProducerUI>,
    mut producer_ui_query: Query<&mut Style, With<ProducerUI>>,
) {
    for _ in remove_producer_ui.read() {
        for mut style in &mut producer_ui_query {
            style.display = Display::None;
        }
    }
}

fn build_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &BuildButton),
        (Changed<Interaction>, With<BuildButton>),
    >,
    mut build_selection: ResMut<BuildSelection>,
) {
    for (interaction, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::Srgba(GREEN_200);
                build_selection.structure_type = button.structure_type.clone();
                build_selection.is_selected = true;
            }
            Interaction::Hovered => {
                border_color.0 = Color::Srgba(GRAY_200);
            }
            Interaction::None => {
                border_color.0 = Color::Srgba(GRAY_800);
            }
        }
    }
}

fn producer_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<ProducerButton>),
    >,
    selected_structures: Res<SelectedStructures>,
    mut producer_query: Query<&mut Producer>,
    mut faith: ResMut<Faith>,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::Srgba(GREEN_200);

                for entity in selected_structures.entities.clone() {
                    if let Ok(mut producer) = producer_query.get_mut(entity) {
                        if faith.value > producer.cost {
                            producer.queue += 1;
                            faith.value -= producer.cost;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                border_color.0 = Color::Srgba(GRAY_200);
            }
            Interaction::None => {
                border_color.0 = Color::Srgba(GRAY_800);
            }
        }
    }
}

fn producer_queue_display(
    mut text_query: Query<&mut Text, With<QueueText>>,
    selected_structures: Res<SelectedStructures>,
    producer_query: Query<&Producer>,
) {
    for mut text in &mut text_query {
        for entity in selected_structures.entities.clone() {
            if let Ok(producer) = producer_query.get(entity) {
                text.sections[0].value = if producer.queue > 0 {
                    producer.queue.to_string()
                } else {
                    "".into()
                }
            }
        }
    }
}
