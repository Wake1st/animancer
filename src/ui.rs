use bevy::{
    color::palettes::{
        css::{BLACK, DARK_GREEN, DARK_SLATE_BLUE},
        tailwind::{GRAY_200, GRAY_800, GREEN_200},
    },
    prelude::*,
};

use crate::{
    inputs::{mouse_is_hovered_over, BuildSelection},
    producer::{
        AttemptProductionIncrease, DisplayProducerUI, Producer, Production, ProductionType,
        RemoveProducerUI, PRIEST_COST, WARRIOR_COST, WORKER_COST,
    },
    schedule::InGameSet,
    selectable::{SelectedStructures, SelectionState, SelectionStateChanged, SelectionType},
    structure::{StructureType, PRODUCER_ASSET_PATH, SIMPLE_SHRINE_ASSET_PATH},
    teams::TeamType,
    unit::{GOOD_PRIEST_ASSET_PATH, GOOD_WARRIOR_ASSET_PATH, GOOD_WORKER_ASSET_PATH},
    worker::{DisplayWorkerUI, RemoveWorkerUI},
};

const UI_BASE_HEIGHT: f32 = 88.;

const MARGIN: Val = Val::Px(12.);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.35, 0.35);
const COST_TEXT_COLOR: Color = Color::Srgba(BLACK);

pub const SIMPLE_SHRINE_COST: f32 = 60.;
pub const PRODUCER_COST: f32 = 140.;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        app.add_systems(Startup, (setup_ui_base, setup_worker_ui, setup_producer_ui))
            .add_systems(
                Update,
                (
                    update_ui,
                    (remove_worker_ui, display_worker_ui).chain(),
                    (remove_producer_ui, display_producer_ui).chain(),
                )
                    .chain()
                    .after(InGameSet::UserInput),
            )
            .add_systems(
                Update,
                (
                    (build_button_interactions, producer_button_interactions)
                        .run_if(mouse_is_hovered_over::<false>),
                    production_queue_display,
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
    cost: f32,
}

#[derive(Component)]
struct ProducerButton {
    pub production_type: ProductionType,
}

#[derive(Component)]
struct QueueText {
    pub production_type: ProductionType,
}

#[derive(Component)]
pub struct WorkerUI {}

#[derive(Component)]
pub struct ProducerUI {}

#[derive(Debug)]
pub enum UIType {
    None,
    Worker,
    Producer,
    Generator,
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
    let producer_texture: Handle<Image> = asset_server.load(PRODUCER_ASSET_PATH);

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
                                    texture: simple_shrine_texture,
                                    ..default()
                                },
                                ..default()
                            },
                            BuildButton {
                                structure_type: StructureType::SimpleShrine,
                                cost: SIMPLE_SHRINE_COST,
                            },
                        ))
                        .with_children(|builder| {
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        padding: UiRect::all(Val::Px(2.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::FlexEnd,
                                        align_items: AlignItems::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    builder.spawn(TextBundle {
                                        text: Text::from_section(
                                            SIMPLE_SHRINE_COST.to_string(),
                                            TextStyle {
                                                color: COST_TEXT_COLOR,
                                                ..default()
                                            },
                                        ),
                                        ..default()
                                    });
                                });
                        });

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
                                    texture: producer_texture,
                                    ..default()
                                },
                                ..default()
                            },
                            BuildButton {
                                structure_type: StructureType::Producer,
                                cost: PRODUCER_COST,
                            },
                        ))
                        .with_children(|builder| {
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        padding: UiRect::all(Val::Px(2.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::FlexEnd,
                                        align_items: AlignItems::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    builder.spawn(TextBundle {
                                        text: Text::from_section(
                                            PRODUCER_COST.to_string(),
                                            TextStyle {
                                                color: COST_TEXT_COLOR,
                                                ..default()
                                            },
                                        ),
                                        ..default()
                                    });
                                });
                        });
                });
        });
}

fn setup_producer_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let worker_texture: Handle<Image> = asset_server.load(GOOD_WORKER_ASSET_PATH);
    let priest_texture: Handle<Image> = asset_server.load(GOOD_PRIEST_ASSET_PATH);
    let warrior_texture: Handle<Image> = asset_server.load(GOOD_WARRIOR_ASSET_PATH);

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
                    production_button(
                        builder,
                        worker_texture,
                        ProductionType::Worker,
                        WORKER_COST.to_string(),
                    );
                    production_button(
                        builder,
                        priest_texture,
                        ProductionType::Priest,
                        PRIEST_COST.to_string(),
                    );
                    production_button(
                        builder,
                        warrior_texture,
                        ProductionType::Warrior,
                        WARRIOR_COST.to_string(),
                    );
                });
        });
}

fn production_button(
    parent: &mut ChildBuilder,
    texture: Handle<Image>,
    production_type: ProductionType,
    cost_text: String,
) {
    parent
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
                    texture,
                    ..default()
                },
                ..default()
            },
            ProducerButton {
                production_type: production_type.clone(),
            },
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
                        QueueText { production_type },
                    ));
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            cost_text,
                            TextStyle {
                                color: COST_TEXT_COLOR,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn update_ui(
    mut selection_state_changed: EventReader<SelectionStateChanged>,
    mut selection_state: ResMut<SelectionState>,
    mut current_ui: ResMut<CurrentUI>,
    mut remove_worker_ui: EventWriter<RemoveWorkerUI>,
    mut display_worker_ui: EventWriter<DisplayWorkerUI>,
    mut remove_producer_ui: EventWriter<RemoveProducerUI>,
    mut display_producer_ui: EventWriter<DisplayProducerUI>,
) {
    for selection_change in selection_state_changed.read() {
        if selection_change.team != TeamType::Human {
            continue;
        }

        selection_state.0 = selection_change.new_type.clone();

        match (&selection_change.new_type, &current_ui.ui_type) {
            (SelectionType::None, UIType::None) => {
                current_ui.ui_type = UIType::None;
            }
            (SelectionType::None, UIType::Worker) => {
                remove_worker_ui.send(RemoveWorkerUI {});
                current_ui.ui_type = UIType::None;
            }
            (SelectionType::None, UIType::Producer) => {
                remove_producer_ui.send(RemoveProducerUI {});
                current_ui.ui_type = UIType::None;
            }
            (SelectionType::None, UIType::Generator) => {
                current_ui.ui_type = UIType::None;
            }
            (SelectionType::Unit, UIType::None) => (),
            (SelectionType::Unit, UIType::Worker) => (),
            (SelectionType::Unit, UIType::Producer) => (),
            (SelectionType::Unit, UIType::Generator) => (),
            (SelectionType::Worker, UIType::None) => {
                display_worker_ui.send(DisplayWorkerUI {});
                current_ui.ui_type = UIType::Worker;
            }
            (SelectionType::Worker, UIType::Worker) => (),
            (SelectionType::Worker, UIType::Producer) => {
                remove_producer_ui.send(RemoveProducerUI {});
                display_worker_ui.send(DisplayWorkerUI {});
                current_ui.ui_type = UIType::Worker;
            }
            (SelectionType::Worker, UIType::Generator) => {
                display_worker_ui.send(DisplayWorkerUI {});
                current_ui.ui_type = UIType::Worker;
            }
            (SelectionType::Priest, UIType::None) => (),
            (SelectionType::Priest, UIType::Worker) => (),
            (SelectionType::Priest, UIType::Producer) => (),
            (SelectionType::Priest, UIType::Generator) => (),
            (SelectionType::Warrior, UIType::None) => (),
            (SelectionType::Warrior, UIType::Worker) => (),
            (SelectionType::Warrior, UIType::Producer) => (),
            (SelectionType::Warrior, UIType::Generator) => (),
            (SelectionType::Construction, UIType::None) => (),
            (SelectionType::Construction, UIType::Worker) => (),
            (SelectionType::Construction, UIType::Producer) => (),
            (SelectionType::Construction, UIType::Generator) => (),
            (SelectionType::Generator, UIType::None) => {
                current_ui.ui_type = UIType::Generator;
            }
            (SelectionType::Generator, UIType::Worker) => {
                remove_worker_ui.send(RemoveWorkerUI {});
                current_ui.ui_type = UIType::Generator;
            }
            (SelectionType::Generator, UIType::Producer) => {
                remove_producer_ui.send(RemoveProducerUI {});
                current_ui.ui_type = UIType::Generator;
            }
            (SelectionType::Generator, UIType::Generator) => (),
            (SelectionType::Producer, UIType::None) => {
                display_producer_ui.send(DisplayProducerUI {});
                current_ui.ui_type = UIType::Producer;
            }
            (SelectionType::Producer, UIType::Worker) => {
                remove_worker_ui.send(RemoveWorkerUI {});
                display_producer_ui.send(DisplayProducerUI {});
                current_ui.ui_type = UIType::Producer;
            }
            (SelectionType::Producer, UIType::Producer) => (),
            (SelectionType::Producer, UIType::Generator) => {
                display_producer_ui.send(DisplayProducerUI {});
                current_ui.ui_type = UIType::Producer;
            }
        }
    }
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
    mut update_selection_state: EventWriter<SelectionStateChanged>,
) {
    for (interaction, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::Srgba(GREEN_200);

                build_selection.structure_type = button.structure_type.clone();
                build_selection.is_selected = true;
                build_selection.cost = button.cost;

                update_selection_state.send(SelectionStateChanged {
                    new_type: SelectionType::Construction,
                    team: TeamType::Human,
                });
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
        (&Interaction, &mut BorderColor, &ProducerButton),
        (Changed<Interaction>, With<ProducerButton>),
    >,
    mut attempt_production_event: EventWriter<AttemptProductionIncrease>,
) {
    for (interaction, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::Srgba(GREEN_200);

                attempt_production_event.send(AttemptProductionIncrease {
                    production_type: button.production_type.clone(),
                    team: TeamType::Human,
                });
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

fn production_queue_display(
    mut text_query: Query<(&mut Text, &QueueText)>,
    selected_structures: Res<SelectedStructures>,
    producer_query: Query<&Children, With<Producer>>,
    production_query: Query<&Production>,
) {
    for entity in selected_structures.entities.human.clone() {
        if let Ok(children) = producer_query.get(entity) {
            for &child in children.iter() {
                if let Ok(production) = production_query.get(child) {
                    for (mut text, queue_text) in &mut text_query {
                        if queue_text.production_type == production.production_type {
                            text.sections[0].value = if production.queue > 0 {
                                production.queue.to_string()
                            } else {
                                "".into()
                            }
                        }
                    }
                }
            }
        }
    }
}
