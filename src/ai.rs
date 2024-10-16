use bevy::{math::vec2, prelude::*};

use crate::{
    construction::PlaceConstructionSite,
    currency::Energy,
    movement::{Formation, Moving, SetUnitPosition},
    producer::{Producer, Production, ProductionType},
    schedule::InGameSet,
    selectable::{BoxSelection, SelectedStructures, SelectedUnits},
    structure::StructureType,
    teams::TeamType,
    ui::{PRODUCER_COST, SIMPLE_SHRINE_COST},
    unit::Unit,
};

const AI_STEP_COOLDOWN: f32 = 1.0;

const CORNER_OFFSET: Vec2 = Vec2::new(5000., 5000.);

const GENERATOR_1_POSITION: Vec2 = Vec2::new(400., 400.);
const GENERATOR_2_POSITION: Vec2 = Vec2::new(400., 300.);
const GENERATOR_3_POSITION: Vec2 = Vec2::new(300., 400.);
const GENERATOR_4_POSITION: Vec2 = Vec2::new(300., 300.);

const PRODUCER_1_POSITION: Vec2 = Vec2::new(800., 600.);

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_instructions)
            .add_systems(Update, run_instruction.in_set(InGameSet::AIInput))
            .add_event::<RunNextInstruction>()
            .insert_resource(AIInstructionSets {
                current_phase: 0,
                cooldown: AI_STEP_COOLDOWN,
                sets: Vec::new(),
            });
    }
}

#[derive(Component)]
pub struct Idle(pub bool);

pub struct Dependancy {
    pub entity: Entity,
    pub expectation: DependancyExpectation,
}

pub enum DependancyExpectation {
    Idle(bool),
    Moving(bool),
}

#[derive(Debug)]
pub enum AIInstructionType {
    Selection(Rect),
    Movement(Vec2),
    Build {
        position: Vec2,
        structure: StructureType,
        cost: f32,
    },
    Worship,
    Produce {
        production: ProductionType,
        count: usize,
    },
}

pub struct AIInstructionSet {
    pub name: String,
    pub phase: usize,
    pub current_step: usize,
    pub complete: bool,
    pub steps: Vec<AIInstructionType>,
    pub dependants: Vec<Dependancy>,
}

impl Default for AIInstructionSet {
    fn default() -> Self {
        Self {
            name: default(),
            phase: default(),
            current_step: 0,
            complete: false,
            steps: default(),
            dependants: default(),
        }
    }
}

#[derive(Resource)]
pub struct AIInstructionSets {
    pub current_phase: usize,
    pub sets: Vec<AIInstructionSet>,
    pub cooldown: f32,
}

#[derive(Event)]
pub struct RunNextInstruction {}

fn load_instructions(mut instruction_sets: ResMut<AIInstructionSets>) {
    *instruction_sets = AIInstructionSets {
        current_phase: 0,
        cooldown: AI_STEP_COOLDOWN,
        sets: vec![
            AIInstructionSet {
                name: "build 1st generator".into(),
                phase: 0,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - vec2(590., 590.),
                        CORNER_OFFSET - vec2(610., 610.),
                    )),
                    AIInstructionType::Build {
                        position: CORNER_OFFSET - GENERATOR_1_POSITION,
                        structure: StructureType::SimpleShrine,
                        cost: SIMPLE_SHRINE_COST,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "build 2nd generator".into(),
                phase: 1,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - GENERATOR_1_POSITION - vec2(80., 80.),
                        CORNER_OFFSET - GENERATOR_1_POSITION + vec2(80., 80.),
                    )),
                    AIInstructionType::Build {
                        position: CORNER_OFFSET - GENERATOR_2_POSITION,
                        structure: StructureType::SimpleShrine,
                        cost: SIMPLE_SHRINE_COST,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "build 1st producer".into(),
                phase: 2,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - GENERATOR_2_POSITION - vec2(80., 80.),
                        CORNER_OFFSET - GENERATOR_2_POSITION + vec2(80., 80.),
                    )),
                    AIInstructionType::Build {
                        position: CORNER_OFFSET - PRODUCER_1_POSITION,
                        structure: StructureType::Producer,
                        cost: PRODUCER_COST,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "produce 1st worker".into(),
                phase: 3,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - PRODUCER_1_POSITION,
                        CORNER_OFFSET - PRODUCER_1_POSITION,
                    )),
                    AIInstructionType::Produce {
                        production: ProductionType::Worker,
                        count: 1,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "build 3rd generator".into(),
                phase: 4,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - PRODUCER_1_POSITION - vec2(80., 80.),
                        CORNER_OFFSET - PRODUCER_1_POSITION + vec2(80., 80.),
                    )),
                    AIInstructionType::Build {
                        position: CORNER_OFFSET - GENERATOR_3_POSITION,
                        structure: StructureType::SimpleShrine,
                        cost: SIMPLE_SHRINE_COST,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "build 4th generator".into(),
                phase: 5,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - GENERATOR_3_POSITION - vec2(60., 60.),
                        CORNER_OFFSET - GENERATOR_3_POSITION + vec2(60., 60.),
                    )),
                    AIInstructionType::Build {
                        position: CORNER_OFFSET - GENERATOR_4_POSITION,
                        structure: StructureType::SimpleShrine,
                        cost: SIMPLE_SHRINE_COST,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "create 3 warriors".into(),
                phase: 6,
                steps: vec![AIInstructionType::Produce {
                    production: ProductionType::Warrior,
                    count: 3,
                }],
                ..default()
            },
        ],
    };
}

fn run_instruction(
    time: Res<Time>,
    mut instruction_sets: ResMut<AIInstructionSets>,
    mut box_selection: EventWriter<BoxSelection>,
    mut set_unit_position: EventWriter<SetUnitPosition>,
    mut place_construction_site: EventWriter<PlaceConstructionSite>,
    selected_units: Res<SelectedUnits>,
    selected_structures: Res<SelectedStructures>,
    mut idlers_query: Query<&mut Idle, With<Idle>>,
    mut movers_query: Query<&mut Moving, With<Unit>>,
    mut producer_query: Query<(&mut Producer, &Children)>,
    mut production_query: Query<&mut Production>,
    mut energy: ResMut<Energy>,
) {
    //  allow some time between instructions
    instruction_sets.cooldown -= time.delta_seconds();
    if instruction_sets.cooldown > 0.0 {
        return;
    }
    instruction_sets.cooldown = AI_STEP_COOLDOWN;

    //  store high scope info
    let current = instruction_sets.current_phase.clone();
    let mut phase_ongoing = false;

    for set in instruction_sets.sets.iter_mut() {
        if !set.complete && set.phase == current {
            phase_ongoing = true;
            info!("running set: {:?}", set.name);

            //  perform step if no remaining dependants
            if set.dependants.is_empty() {
                //  execute step
                let index = &set.current_step;
                let step = &set.steps[*index];
                info!("running ai step: {:?}", step);

                match &step {
                    AIInstructionType::Selection(rect) => {
                        box_selection.send(BoxSelection {
                            rect: *rect,
                            team: TeamType::CPU,
                        });
                    }
                    AIInstructionType::Movement(position) => {
                        //  set move orders
                        set_unit_position.send(SetUnitPosition {
                            position: *position,
                            direction: Vec2::ZERO,
                            formation: Formation::Ringed,
                            ..default()
                        });

                        //  add dependants
                        establish_moving_dependants(
                            selected_units.entities.clone(),
                            set,
                            &mut movers_query,
                        );
                    }
                    AIInstructionType::Build {
                        position,
                        structure,
                        cost,
                    } => {
                        //  place site
                        info!("building at: {:?}", *position);
                        place_construction_site.send(PlaceConstructionSite {
                            structure_type: structure.clone(),
                            position: *position,
                            effort: *cost,
                        });

                        //  ensure units move to build
                        set_unit_position.send(SetUnitPosition {
                            position: *position,
                            direction: Vec2::ONE * 40.0,
                            formation: Formation::Ringed,
                        });

                        //  add dependants
                        establish_idle_dependants(
                            selected_units.entities.clone(),
                            set,
                            &mut idlers_query,
                        );
                    }
                    AIInstructionType::Worship => todo!(),
                    AIInstructionType::Produce { production, count } => {
                        let mut producing: bool = false;

                        for _ in 0..*count {
                            for entity in selected_structures.entities.clone() {
                                if let Ok((mut producer, children)) = producer_query.get_mut(entity)
                                {
                                    for &child in children.iter() {
                                        if let Ok(mut prod) = production_query.get_mut(child) {
                                            if prod.production_type == *production
                                                && energy.value > prod.cost
                                            {
                                                energy.value -= prod.cost;
                                                prod.queue += 1;

                                                producer.queue.push(prod.production_type.clone());

                                                //  only set if it's the first
                                                if producer.queue.len() == 1 {
                                                    producer.current_production =
                                                        producer.queue[0].clone();
                                                }

                                                producing = true;
                                                info!("actually producing");
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if producing {
                            //  add dependants
                            establish_idle_dependants(
                                selected_structures.entities.clone(),
                                set,
                                &mut idlers_query,
                            );
                        }
                    }
                }
            }

            //  check dependants
            let mut removing: Vec<usize> = Vec::new();
            for (index, dependancy) in set.dependants.iter().enumerate() {
                let remove = match dependancy.expectation {
                    DependancyExpectation::Idle(b) => {
                        if let Ok(idle) = idlers_query.get(dependancy.entity) {
                            idle.0 == b
                        } else {
                            false
                        }
                    }
                    DependancyExpectation::Moving(b) => {
                        if let Ok(moving) = movers_query.get(dependancy.entity) {
                            moving.0 == b
                        } else {
                            false
                        }
                    }
                };

                if remove {
                    removing.push(index);
                }
            }

            for &index in removing.iter() {
                set.dependants.swap_remove(index);
            }

            if set.dependants.is_empty() {
                //  next step or leave
                set.current_step += 1;
                if set.current_step == set.steps.len() {
                    set.complete = true;
                }
            }
        }
    }

    if !phase_ongoing {
        instruction_sets.current_phase += 1;
    }
}

fn establish_idle_dependants(
    entities: Vec<Entity>,
    set: &mut AIInstructionSet,
    idlers_query: &mut Query<&mut Idle, With<Idle>>,
) {
    for &entity in entities.iter() {
        if let Ok(mut idle) = idlers_query.get_mut(entity) {
            set.dependants.push(Dependancy {
                entity,
                expectation: DependancyExpectation::Idle(true),
            });
            idle.0 = false;

            info!("adding {:?} as an idle dependancy", entity);
        }
    }
}

fn establish_moving_dependants(
    entities: Vec<Entity>,
    set: &mut AIInstructionSet,
    movers_query: &mut Query<&mut Moving, With<Unit>>,
) {
    for &entity in entities.iter() {
        if let Ok(mut idle) = movers_query.get_mut(entity) {
            set.dependants.push(Dependancy {
                entity,
                expectation: DependancyExpectation::Moving(false),
            });
            idle.0 = false;
        }
    }
}
