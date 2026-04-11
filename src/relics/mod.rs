mod defs;

use std::{collections::HashMap, sync::LazyLock};

use crate::{EncounterOp, cards::CardInstance, encounters::{self, Encounter}, relics::defs::*};

pub static RELICS: LazyLock<HashMap<Relics, &'static RelicImpl>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Relics::RingOfTheSnake, &RING_OF_THE_SNAKE);
    m.insert(Relics::BloodVial, &BLOOD_VIAL);
    m.insert(Relics::Vajra, &VAJRA);
    m.insert(Relics::Tingsha, &TINGSHA);
    m
});

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Relics {
    RingOfTheSnake,
    BloodVial,
    Vajra,
    Tingsha
}

pub type CombatStartHandler = fn(encounter: &Encounter) -> Vec<EncounterOp>;
pub type DiscardHandler = fn(card: &CardInstance, encounter: &Encounter) -> Vec<EncounterOp>;

pub struct RelicImpl {
    pub combat_started: Option<CombatStartHandler> = None,
    pub card_discarded: Option<DiscardHandler> = None
}
