use bevy::prelude::*;

use crate::{schedule::InGameSet, selectable::SelectionType};

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
    Selection,
    Movement,
    Confront,
    Build,
    Worship,
    Produce,
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
                    AIInstructionType::Selection,
                    AIInstructionType::Movement,
                    AIInstructionType::Build,
                ],
                ..default()
            },
            AIInstructionSet {
                name: "build 2nd generator".into(),
                phase: 1,
                steps: vec![
                    AIInstructionType::Selection,
                    AIInstructionType::Movement,
                    AIInstructionType::Build,
                ],
                ..default()
            },
            AIInstructionSet {
                name: "build 1st producer".into(),
                phase: 2,
                steps: vec![
                    AIInstructionType::Selection,
                    AIInstructionType::Movement,
                    AIInstructionType::Build,
                ],
                ..default()
            },
        ],
    };
}

fn run_instruction(mut instruction_sets: ResMut<AIInstructionSets>) {
    let current = instruction_sets.current_phase.clone();
    let mut phase_ongoing = false;

    for mut set in instruction_sets.sets.iter_mut() {
        if !set.complete && set.phase == current {
            phase_ongoing = true;

            if set.active {
                //  perform step
                let index = &set.current_step;
                let step = &set.steps[*index];

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
