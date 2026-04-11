use crate::{Effect, Run, cards::CardInstance, monsters::Enemy};

pub mod encounter;
pub mod run;

pub struct Encounter<'a> {
    pub run: &'a mut Run,
    
    pub player: Player,
    pub draw_pile: Vec<CardInstance>,
    pub hand: Vec<CardInstance>,
    pub discard_pile: Vec<CardInstance>,
    pub exhaust_pile: Vec<CardInstance>,

    pub turn: u32,
    pub enemies: Vec<Enemy>,
}

pub struct Player {
    pub energy: u32,
    pub block: u32,
    pub health: u32,
    pub effects: Vec<Effect>
}