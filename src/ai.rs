use bevy::{math::vec2, prelude::*};

use crate::{
    construction::PlaceConstructionSite,
    currency::Energy,
    movement::SetUnitPosition,
    producer::{Producer, Production, ProductionType},
    schedule::InGameSet,
    selectable::{BoxSelection, SelectedStructures},
    structure::StructureType,
    ui::{PRODUCER_COST, SIMPLE_SHRINE_COST},
};

const CORNER_OFFSET: Vec2 = Vec2::new(5000., 5000.);

const GENERATOR_1_POSITION: Vec2 = Vec2::new(400., 400.);
const GENERATOR_2_POSITION: Vec2 = Vec2::new(400., 370.);
const GENERATOR_3_POSITION: Vec2 = Vec2::new(370., 400.);
const GENERATOR_4_POSITION: Vec2 = Vec2::new(400., 400.);

const PRODUCER_1_POSITION: Vec2 = Vec2::new(800., 600.);

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_instructions)
            .add_systems(Update, run_instruction.in_set(InGameSet::AIInput))
            .add_event::<RunNextInstruction>()
            .insert_resource(AIInstructionSets {
                current_phase: 0,
                sets: Vec::new(),
            });
    }
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
    pub active: bool,
    pub steps: Vec<AIInstructionType>,
}

impl Default for AIInstructionSet {
    fn default() -> Self {
        Self {
            name: Default::default(),
            phase: Default::default(),
            current_step: 0,
            complete: false,
            active: false,
            steps: Default::default(),
        }
    }
}

#[derive(Resource)]
pub struct AIInstructionSets {
    pub current_phase: usize,
    pub sets: Vec<AIInstructionSet>,
}

#[derive(Event)]
pub struct RunNextInstruction {}

fn load_instructions(mut instruction_sets: ResMut<AIInstructionSets>) {
    *instruction_sets = AIInstructionSets {
        current_phase: 0,
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
                        CORNER_OFFSET - GENERATOR_1_POSITION - vec2(10., 10.),
                        CORNER_OFFSET - GENERATOR_1_POSITION + vec2(10., 10.),
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
                        CORNER_OFFSET - GENERATOR_2_POSITION - vec2(10., 10.),
                        CORNER_OFFSET - GENERATOR_2_POSITION + vec2(10., 10.),
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
                        CORNER_OFFSET - PRODUCER_1_POSITION - vec2(50., 50.),
                        CORNER_OFFSET - PRODUCER_1_POSITION + vec2(50., 50.),
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
                        CORNER_OFFSET - GENERATOR_3_POSITION - vec2(50., 50.),
                        CORNER_OFFSET - GENERATOR_3_POSITION + vec2(50., 50.),
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
    mut instruction_sets: ResMut<AIInstructionSets>,
    mut box_selection: EventWriter<BoxSelection>,
    mut set_unit_position: EventWriter<SetUnitPosition>,
    mut place_construction_site: EventWriter<PlaceConstructionSite>,
    selected_structures: Res<SelectedStructures>,
    mut producer_query: Query<(&mut Producer, &Children)>,
    mut production_query: Query<&mut Production>,
    mut energy: ResMut<Energy>,
) {
    let current = instruction_sets.current_phase.clone();
    let mut phase_ongoing = false;

    for set in instruction_sets.sets.iter_mut() {
        if !set.complete && set.phase == current {
            info!("running set: {:?}", set.name);
            phase_ongoing = true;

            if set.active {
                //  perform step
                let index = &set.current_step;
                let step = &set.steps[*index];

                match &step {
                    AIInstructionType::Selection(rect) => {
                        box_selection.send(BoxSelection { rect: *rect });
                    }
                    AIInstructionType::Movement(position) => {
                        set_unit_position.send(SetUnitPosition {
                            position: *position,
                            ..default()
                        });
                    }
                    AIInstructionType::Build {
                        position,
                        structure,
                        cost,
                    } => {
                        place_construction_site.send(PlaceConstructionSite {
                            structure_type: structure.clone(),
                            position: *position,
                            effort: *cost,
                        });
                    }
                    AIInstructionType::Worship => todo!(),
                    AIInstructionType::Produce { production, count } => {
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
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                //  next or final step complete
                set.current_step += 1;
                if set.current_step == set.steps.len() {
                    set.complete = true;
                }
            } else {
                set.active = true;
            }
        }
    }

    if !phase_ongoing {
        instruction_sets.current_phase += 1;
    }
}
