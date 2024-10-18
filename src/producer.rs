use bevy::{math::vec3, prelude::*};

use crate::{
    ai::Idle,
    currency::Energy,
    schedule::InGameSet,
    selectable::SelectedStructures,
    structure::Structure,
    teams::{Team, TeamType},
};

pub const SPAWN_OFFSET: Vec3 = vec3(0.0, -50.0, 0.1);

pub const WORKER_COST: f32 = 10.0;
pub const PRIEST_COST: f32 = 18.0;
pub const WARRIOR_COST: f32 = 14.0;

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (attempt_production_increase, produce).chain(),
                display_post_spawn_marker,
            )
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<AttemptProductionIncrease>()
        .add_event::<Produce>()
        .add_event::<DisplayProducerUI>()
        .add_event::<RemoveProducerUI>();
    }
}

#[derive(Component, Debug)]
pub struct Producer {
    pub current_production: ProductionType,
    pub queue: Vec<ProductionType>,
    pub value: f32,
    pub rate: f32,
    pub post_spawn_location: Vec3,
}

impl Default for Producer {
    fn default() -> Self {
        Self {
            current_production: ProductionType::None,
            queue: Vec::new(),
            value: 0.0,
            rate: 2.0,
            post_spawn_location: Vec3::ZERO,
        }
    }
}

impl Clone for Producer {
    fn clone(&self) -> Self {
        Self {
            current_production: self.current_production.clone(),
            queue: self.queue.clone(),
            value: self.value.clone(),
            rate: self.rate.clone(),
            post_spawn_location: self.post_spawn_location.clone(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Production {
    pub production_type: ProductionType,
    pub cost: f32,
    pub queue: i32,
}

impl Clone for Production {
    fn clone(&self) -> Self {
        Self {
            production_type: self.production_type.clone(),
            cost: self.cost.clone(),
            queue: self.queue.clone(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ProductionType {
    None,
    Worker,
    Priest,
    Warrior,
}

impl Clone for ProductionType {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Worker => Self::Worker,
            Self::Priest => Self::Priest,
            Self::Warrior => Self::Warrior,
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
pub struct AttemptProductionIncrease {
    pub production_type: ProductionType,
    pub team: TeamType,
}

#[derive(Event)]
pub struct Produce {
    pub production_type: ProductionType,
    pub position: Vec3,
    pub location: Vec3,
    pub team: TeamType,
}

fn attempt_production_increase(
    mut attempt_event: EventReader<AttemptProductionIncrease>,
    selected_structures: Res<SelectedStructures>,
    mut producer_query: Query<(&mut Producer, &Children)>,
    mut production_query: Query<&mut Production>,
    mut energy: ResMut<Energy>,
) {
    for attempt in attempt_event.read() {
        let selected_entities = match attempt.team {
            TeamType::Human => selected_structures.entities.human.clone(),
            TeamType::CPU => selected_structures.entities.cpu.clone(),
        };

        for entity in selected_entities {
            if let Ok((mut producer, children)) = producer_query.get_mut(entity) {
                for &child in children.iter() {
                    if let Ok(mut production) = production_query.get_mut(child) {
                        // info!(
                        //     "\tproduction type: {:?} ?? {:?} :attempt type\n\tenergy: {:?} >? {:?} :cost",
                        //     production.production_type, attempt.production_type,energy.value, production.cost
                        // );
                        if production.production_type == attempt.production_type
                            && energy.value > production.cost
                        {
                            energy.value -= production.cost;
                            production.queue += 1;

                            producer.queue.push(production.production_type.clone());

                            //  only set if it's the first
                            if producer.queue.len() == 1 {
                                producer.current_production = producer.queue[0].clone();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn produce(
    time: Res<Time>,
    mut producer_query: Query<
        (&mut Producer, &GlobalTransform, &Children, &mut Idle, &Team),
        With<Structure>,
    >,
    mut production_query: Query<&mut Production>,
    mut producer_writer: EventWriter<Produce>,
) {
    let delta_time = time.delta_seconds();

    for (mut producer, transform, children, mut idle, team) in producer_query.iter_mut() {
        let current_production = producer.current_production.clone();

        if producer.queue.len() > 0 {
            for &child in children.iter() {
                if let Ok(mut production) = production_query.get_mut(child) {
                    if production.production_type == current_production {
                        producer.value += producer.rate * delta_time;
                        // info!(
                        //     "producer [ value: {:?}\t rate: {:?} ]",
                        //     producer.value, producer.rate
                        // );
                        if producer.value >= production.cost {
                            //	leave the remainder, so as to avoid value loss over time
                            producer.value = producer.value % production.cost;
                            production.queue -= 1;

                            //  create unit
                            producer_writer.send(Produce {
                                production_type: producer.current_production.clone(),
                                position: transform.translation() + SPAWN_OFFSET,
                                location: producer.post_spawn_location,
                                team: team.0.clone(),
                            });

                            //  shift production
                            producer.queue.remove(0);
                            if producer.queue.len() == 0 {
                                producer.current_production = ProductionType::None;
                                idle.0 = true;
                            } else {
                                producer.current_production = producer.queue[0].clone();
                            }
                        }
                    }
                }
            }
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

    for &selected_entity in selected_structures.entities.human.iter() {
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
