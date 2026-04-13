use crate::{Effect, EncounterOp, Run, cards::CardInstance, monsters::Enemy};

pub mod encounter;
pub mod run;

#[derive(Clone, PartialEq, Hash)]
pub struct Encounter {
    pub run: Run,
    
    pub player: Player,
    pub draw_pile: Vec<CardInstance>,
    pub hand: Vec<CardInstance>,
    pub discard_pile: Vec<CardInstance>,
    pub exhaust_pile: Vec<CardInstance>,

    pub this_turn: Vec<EncounterOp>,

    pub turn: u32,
    pub enemies: Vec<Enemy>,
}

#[derive(Clone, PartialEq, Hash)]
pub struct Player {
    pub energy: u32,
    pub block: u32,
    pub health: u32,
    pub effects: Vec<Effect>
}

impl Eq for Encounter {}