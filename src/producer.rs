use bevy::{math::vec3, prelude::*};

use crate::{
    schedule::InGameSet,
    selectable::SelectedStructures,
    structure::Structure,
    ui::{CurrentUI, UIType},
};

pub const SPAWN_OFFSET: Vec3 = vec3(0.0, -40.0, 0.1);

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_producer_ui)
            .add_systems(
                Update,
                (produce, display_post_spawn_marker).in_set(InGameSet::EntityUpdates),
            )
            .add_event::<ProduceWorker>()
            .add_event::<DisplayProducerUI>()
            .add_event::<RemoveProducerUI>();
    }
}

#[derive(Component)]
pub struct Producer {
    pub queue: i32,
    pub cost: f32,
    pub value: f32,
    pub rate: f32,
    pub post_spawn_location: Vec3,
}

impl Default for Producer {
    fn default() -> Self {
        Self {
            queue: 0,
            cost: 10.0,
            value: 0.0,
            rate: 2.0,
            post_spawn_location: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct PostSpawnMarker {
    pub not_set: bool,
}

#[derive(Event)]
pub struct DisplayProducerUI {}

#[derive(Event)]
pub struct RemoveProducerUI {}

#[derive(Event)]
pub struct ProduceWorker {
    pub position: Vec3,
    pub location: Vec3,
}

fn produce(
    time: Res<Time>,
    mut query: Query<(&mut Producer, &GlobalTransform), With<Structure>>,
    mut producer_writer: EventWriter<ProduceWorker>,
) {
    let delta_time = time.delta_seconds();

    for (mut producer, transform) in query.iter_mut() {
        if producer.queue > 0 {
            producer.value += producer.rate * delta_time;

            if producer.value >= producer.cost {
                //	leave the remainder, so as to avoid value loss over time
                producer.value = producer.value % producer.cost;
                producer.queue -= 1;

                producer_writer.send(ProduceWorker {
                    position: transform.translation() + SPAWN_OFFSET,
                    location: producer.post_spawn_location,
                });
            }
        }
    }
}

fn display_producer_ui(
    selected_structures: Res<SelectedStructures>,
    mut current_ui: ResMut<CurrentUI>,
    mut event_writer: EventWriter<DisplayProducerUI>,
) {
    if selected_structures.entities.len() > 0 {
        match current_ui.ui_type {
            UIType::None => {
                event_writer.send(DisplayProducerUI {});
                current_ui.ui_type = UIType::Producer;
            }
            UIType::Worker => (),
            UIType::Producer => (),
        }
    }
}

fn display_post_spawn_marker(
    selected_structures: Res<SelectedStructures>,
    producer_query: Query<(&GlobalTransform, &Producer, &Children)>,
    mut marker_query: Query<
        (&mut Visibility, &mut Transform, &PostSpawnMarker),
        With<PostSpawnMarker>,
    >,
) {
    for (mut visibility, _, _) in marker_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }

    for &selected_entity in selected_structures.entities.iter() {
        if let Ok((global_transform, producer, children)) = producer_query.get(selected_entity) {
            for &child in children.iter() {
                if let Ok((mut visibility, mut transform, marker)) = marker_query.get_mut(child) {
                    if !marker.not_set {
                        *visibility = Visibility::Visible;

                        transform.translation =
                            producer.post_spawn_location - global_transform.translation();
                    }
                }
            }
        }
    }
}
