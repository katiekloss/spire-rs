

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

#[derive(Clone)]
pub enum Effect {
    Strength(u32)
}

pub trait Relic {

}

pub trait Damageable {
    fn get_block(&self) -> u32;
    fn get_health(&self) -> u32;
    fn set_block(&mut self, block: u32);
    fn set_health(&mut self, health: u32);
}

pub trait Effectable {
    fn get_effects(&self) -> &Vec<Effect>;
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