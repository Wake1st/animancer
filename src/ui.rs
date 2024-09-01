use bevy::{
    color::palettes::{
        css::DARK_GREEN,
        tailwind::{GRAY_200, GRAY_800, GREEN_200},
    },
    prelude::*,
};

use crate::{inputs::BuildSelection, schedule::InGameSet, structure::StructureType};

const MARGIN: Val = Val::Px(12.);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.35, 0.35);

const SIMPLE_SHRINE_ASSET_PATH: &str = "harvester.png";
const WORKER_PRODUCER_ASSET_PATH: &str = "worker producer.png";

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        app.add_systems(Startup, spawn_layout)
            .add_systems(Update, button_system.in_set(InGameSet::UIInput))
            .insert_resource(OverUI { value: false });
    }
}

#[derive(Component)]
struct BuildButton {
    structure_type: StructureType,
}

#[derive(Resource)]
pub struct OverUI {
    pub value: bool,
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let simple_shrine_texture: Handle<Image> = asset_server.load(SIMPLE_SHRINE_ASSET_PATH);
    let worker_producer_texture: Handle<Image> = asset_server.load(WORKER_PRODUCER_ASSET_PATH);

    commands
        .spawn(NodeBundle {
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
        })
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

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &BuildButton),
        (Changed<Interaction>, With<BuildButton>),
    >,
    mut build_selection: ResMut<BuildSelection>,
    mut over_ui: ResMut<OverUI>,
) {
    over_ui.value = false;
    for (interaction, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::Srgba(GREEN_200);
                build_selection.structure_type = button.structure_type.clone();
                build_selection.is_selected = true;
                over_ui.value = true;
            }
            Interaction::Hovered => {
                border_color.0 = Color::Srgba(GRAY_200);
                over_ui.value = true;
            }
            Interaction::None => {
                border_color.0 = Color::Srgba(GRAY_800);
                over_ui.value = over_ui.value || false;
            }
        }
    }
}
