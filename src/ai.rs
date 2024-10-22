use bevy::{math::vec2, prelude::*};
use rand::prelude::*;

use crate::{
    construction::PlaceConstructionSite,
    movement::{Formation, Moving, SetUnitPosition},
    producer::{AttemptProductionIncrease, ProductionType},
    schedule::InGameSet,
    selectable::{BoxSelection, SelectedStructures, SelectedUnits},
    structure::StructureType,
    teams::TeamType,
    ui::{PRODUCER_COST, SIMPLE_SHRINE_COST},
    unit::Unit,
};

const AI_STEP_COOLDOWN: f32 = 5.0;
const AI_FORCE_FORWARD: f32 = 30.0;
const AI_FINAL_PHASE: usize = 8;
const AI_RESET_PHASE: usize = 7;

const CORNER_OFFSET: Vec2 = Vec2::new(5000., 5000.);

const GENERATOR_1_POSITION: Vec2 = Vec2::new(400., 400.);
const GENERATOR_2_POSITION: Vec2 = Vec2::new(400., 300.);
const GENERATOR_3_POSITION: Vec2 = Vec2::new(300., 400.);
const GENERATOR_4_POSITION: Vec2 = Vec2::new(300., 300.);
const PRODUCER_1_POSITION: Vec2 = Vec2::new(800., 800.);
const ENEMY_BASE_POSITION: Vec2 = Vec2::new(700., 700.);

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_instructions)
            .add_systems(Update, run_instruction.in_set(InGameSet::AIInput))
            .add_event::<RunNextInstruction>()
            .insert_resource(AIInstructionSets {
                current_phase: 0,
                cooldown: AI_STEP_COOLDOWN,
                force_forward: AI_FORCE_FORWARD,
                sets: Vec::new(),
            });
    }
}

#[derive(Component, Debug)]
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
    pub force_forward: f32,
}

#[derive(Event)]
pub struct RunNextInstruction {}

fn load_instructions(mut instruction_sets: ResMut<AIInstructionSets>) {
    *instruction_sets = AIInstructionSets {
        current_phase: 0,
        cooldown: AI_STEP_COOLDOWN,
        force_forward: AI_FORCE_FORWARD,
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
                        CORNER_OFFSET - GENERATOR_3_POSITION - vec2(80., 80.),
                        CORNER_OFFSET - GENERATOR_3_POSITION + vec2(80., 80.),
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
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - PRODUCER_1_POSITION,
                        CORNER_OFFSET - PRODUCER_1_POSITION,
                    )),
                    AIInstructionType::Produce {
                        production: ProductionType::Warrior,
                        count: 3,
                    },
                ],
                ..default()
            },
            AIInstructionSet {
                name: "send troops to enemy base".into(),
                phase: 7,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - PRODUCER_1_POSITION - vec2(100., 100.),
                        CORNER_OFFSET - PRODUCER_1_POSITION + vec2(100., 100.),
                    )),
                    AIInstructionType::Movement(ENEMY_BASE_POSITION),
                ],
                ..default()
            },
            AIInstructionSet {
                name: "create 5 warriors".into(),
                phase: 7,
                steps: vec![
                    AIInstructionType::Selection(Rect::from_corners(
                        CORNER_OFFSET - PRODUCER_1_POSITION,
                        CORNER_OFFSET - PRODUCER_1_POSITION,
                    )),
                    AIInstructionType::Produce {
                        production: ProductionType::Warrior,
                        count: 5,
                    },
                ],
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
    mut attempt_production_event: EventWriter<AttemptProductionIncrease>,
) {
    //  allow some time between instructions
    let delta = time.delta_seconds();
    instruction_sets.cooldown -= delta;
    instruction_sets.force_forward -= delta;

    if instruction_sets.cooldown > 0.0 {
        return;
    }

    instruction_sets.cooldown = AI_STEP_COOLDOWN;
    let forward_countdown = instruction_sets.force_forward;
    let mut forwarded = false;

    //  store high scope info
    let current = instruction_sets.current_phase.clone();
    let mut phase_ongoing = false;

    for set in instruction_sets.sets.iter_mut() {
        if !set.complete && set.phase == current {
            phase_ongoing = true;
            debug!("running set: {:?}", set.name);

            //  perform step if no remaining dependants
            if set.dependants.is_empty() {
                //  execute step
                let index = &set.current_step;
                let step = &set.steps[*index];
                debug!("running ai step: {:?}", step);

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
                            position: *position + rand_adjustment(),
                            direction: Vec2::ZERO,
                            formation: Formation::Ringed,
                            team: TeamType::CPU,
                            ..default()
                        });

                        //  add dependants
                        establish_moving_dependants(
                            selected_units.entities.cpu.clone(),
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
                        // info!("building at: {:?}", *position);
                        place_construction_site.send(PlaceConstructionSite {
                            structure_type: structure.clone(),
                            position: *position,
                            team: TeamType::CPU,
                            effort: *cost,
                        });

                        //  ensure units move to build
                        set_unit_position.send(SetUnitPosition {
                            position: *position,
                            direction: Vec2::ONE * 40.0,
                            formation: Formation::Ringed,
                            team: TeamType::CPU,
                        });

                        //  add dependants
                        establish_idle_dependants(
                            selected_units.entities.cpu.clone(),
                            set,
                            &mut idlers_query,
                        );
                    }
                    AIInstructionType::Produce { production, count } => {
                        let mut producing: bool = false;

                        // info!(
                        //     "selected structures: {:?}",
                        //     selected_structures.entities.len()
                        // );
                        for _ in 0..*count {
                            producing = true;

                            attempt_production_event.send(AttemptProductionIncrease {
                                production_type: production.clone(),
                                team: TeamType::CPU,
                            });
                        }

                        if producing {
                            //  add dependants
                            establish_idle_dependants(
                                selected_structures.entities.cpu.clone(),
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

            let mut index_adjustment: usize = 0;
            for &index in removing.iter() {
                set.dependants.swap_remove(index - index_adjustment);
                index_adjustment += 1;
            }

            if set.dependants.is_empty() || forward_countdown < 0.0 {
                //  next step or leave
                set.current_step += 1;
                if set.current_step == set.steps.len() {
                    set.complete = true;
                }

                forwarded = true;
            }
        }
    }

    if !phase_ongoing {
        instruction_sets.current_phase += 1;

        //  final instructions loop endlessly
        if instruction_sets.current_phase == AI_FINAL_PHASE {
            instruction_sets.current_phase = AI_RESET_PHASE;

            for set in instruction_sets.sets.iter_mut() {
                set.complete = false;
                set.current_step = 0;
            }
        }
    }

    if forwarded {
        instruction_sets.force_forward = AI_FORCE_FORWARD;
    }
}

fn rand_adjustment() -> Vec2 {
    let mut rnd = rand::thread_rng();
    let range = 1000.0;
    vec2(
        (rnd.gen::<f32>() - 0.5) * range,
        (rnd.gen::<f32>() - 0.5) * range,
    )
}

fn establish_idle_dependants(
    entities: Vec<Entity>,
    set: &mut AIInstructionSet,
    idlers_query: &mut Query<&mut Idle, With<Idle>>,
) {
    for &entity in entities.iter() {
        if let Ok(mut idle) = idlers_query.get_mut(entity) {
            // info!("adding {:?} as an idle dependancy({:?})", entity, idle);

            set.dependants.push(Dependancy {
                entity,
                expectation: DependancyExpectation::Idle(true),
            });
            idle.0 = false;
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
