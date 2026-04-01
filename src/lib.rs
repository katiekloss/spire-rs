
use crate::{cards::CardInstance, relics::Relics};

pub mod cards;
pub mod relics;
pub mod monsters;
pub mod encounters;

pub struct Run {
    pub floor: u32,
    pub relics: Vec<Relics>,
    pub health: u32,
    pub gold: u32,
    pub deck: Vec<CardInstance>,
}

pub enum EncounterOutcome {
    Alive,
    Dead
}

pub enum Effect {
    Strength(u32)
}

pub trait Relic {

}

#[derive(Clone, PartialEq)]
pub enum Keywords {
    Eternal,
    Ethereal,
    Exhaust,
    Innate,
    Retain,
    Sly,
    Unplayable
}