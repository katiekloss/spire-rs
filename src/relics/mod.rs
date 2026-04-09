mod defs;

use std::{collections::HashMap, sync::LazyLock};

use crate::{encounters::Encounter, relics::defs::{BLOOD_VIAL, RING_OF_THE_SNAKE}};

pub static RELICS: LazyLock<HashMap<Relics, &'static RelicImpl>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Relics::RingOfTheSnake, &RING_OF_THE_SNAKE);
    m.insert(Relics::BloodVial, &BLOOD_VIAL);
    m
});

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Relics {
    RingOfTheSnake,
    BloodVial
}

pub struct RelicState {
    pub _counter: u32
}

pub type RelicInstance = (Relics, RelicState);

pub type CombatStartHandler = fn(counter: u32, encounter: &mut Encounter) -> u32;

pub struct RelicImpl {
    pub combat_started: Option<CombatStartHandler> = None
}
