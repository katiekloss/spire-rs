use crate::relics::Relics;

pub mod cards;
pub mod relics;
pub mod monsters;
pub mod services;

pub struct Run {
    pub floor: u32,
    pub relics: Vec<Relics>,
    pub health: u32,
    pub gold: u32
}

pub struct EncounterState {
}

pub enum EncounterOutcome {
    Alive,
    Dead
}

pub struct Enemy {
    pub effects: Vec<Box<dyn Effect>>
}

pub trait Effect {

}

pub trait Relic {

}