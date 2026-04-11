mod defs;

use std::{collections::HashMap, sync::LazyLock};

use crate::{EncounterOp, Run, RunOp, cards::CardInstance, core::Encounter, relics::defs::*};

pub static RELICS: LazyLock<HashMap<Relics, &'static RelicImpl>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Relics::RingOfTheSnake, &RING_OF_THE_SNAKE);
    m.insert(Relics::BloodVial, &BLOOD_VIAL);
    m.insert(Relics::Vajra, &VAJRA);
    m.insert(Relics::Tingsha, &TINGSHA);
    m.insert(Relics::Anchor, &ANCHOR);
    m.insert(Relics::BagOfMarbles, &BAG_OF_MARBLES);
    m.insert(Relics::MercuryHourglass, &MERCURY_HOURGLASS);
    m.insert(Relics::Mango, &MANGO);
    m
});

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Relics {
    RingOfTheSnake,
    BloodVial,
    Vajra,
    Tingsha,
    Anchor,
    BagOfMarbles,
    MercuryHourglass,
    Mango
}

pub type PickupHandler = fn(run: &Run) -> Vec<RunOp>;
pub type CombatStartHandler = fn(encounter: &Encounter) -> Vec<EncounterOp>;
pub type TurnStartHandler = fn(encounter: &Encounter) -> Vec<EncounterOp>;
pub type DiscardHandler = fn(card: &CardInstance, encounter: &Encounter) -> Vec<EncounterOp>;

pub struct RelicImpl {
    pub picked_up: Option<PickupHandler> = None,
    pub combat_started: Option<CombatStartHandler> = None,
    pub turn_started: Option<TurnStartHandler> = None,
    pub card_discarded: Option<DiscardHandler> = None
}
